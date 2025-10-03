use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::models::{CreateUserRequest, LoginRequest, User, UserRole};
use crate::utils::{AppError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Subject (user id)
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub exp: i64,          // Expiration time
    pub iat: i64,          // Issued at
}

#[derive(Clone)]
pub struct AuthService {
    jwt_secret: String,
    access_token_expiry: i64,  // in minutes
    refresh_token_expiry: i64, // in minutes
}

impl AuthService {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            jwt_secret,
            access_token_expiry: 60,      // 1 hour
            refresh_token_expiry: 10080,  // 7 days
        }
    }

    /// Hash a password using Argon2
    pub fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::InternalError(format!("Password hashing failed: {}", e)))?
            .to_string()
            .pipe(Ok)
    }

    /// Verify a password against a hash
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::InternalError(format!("Invalid password hash: {}", e)))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Generate JWT access token
    pub fn generate_access_token(&self, user: &User) -> Result<String> {
        let now = Utc::now();
        let expiry = now + Duration::minutes(self.access_token_expiry);

        let claims = Claims {
            sub: user.id.to_string(),
            username: user.username.clone(),
            email: user.email.clone(),
            role: user.role.clone(),
            exp: expiry.timestamp(),
            iat: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalError(format!("Token generation failed: {}", e)))
    }

    /// Generate JWT refresh token (longer expiry)
    pub fn generate_refresh_token(&self, user: &User) -> Result<String> {
        let now = Utc::now();
        let expiry = now + Duration::minutes(self.refresh_token_expiry);

        let claims = Claims {
            sub: user.id.to_string(),
            username: user.username.clone(),
            email: user.email.clone(),
            role: user.role.clone(),
            exp: expiry.timestamp(),
            iat: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalError(format!("Token generation failed: {}", e)))
    }

    /// Validate and decode JWT token
    pub fn validate_token(&self, token: &str) -> Result<TokenData<Claims>> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::TokenExpired,
            _ => AppError::InvalidToken,
        })
    }

    /// Register a new user
    pub async fn register(&self, pool: &PgPool, req: CreateUserRequest) -> Result<User> {
        // Validate request
        req.validate()?;

        // Hash password
        let password_hash = self.hash_password(&req.password)?;

        // Create user in database
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
            RETURNING id, username, email, password_hash, role, created_at, updated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(&req.username)
        .bind(&req.email)
        .bind(&password_hash)
        .bind(req.role.to_string())
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    /// Login user
    pub async fn login(&self, pool: &PgPool, req: LoginRequest) -> Result<User> {
        // Validate request
        req.validate()?;

        // Find user by username
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, username, email, password_hash, role, created_at, updated_at
            FROM users
            WHERE username = $1
            "#,
        )
        .bind(&req.username)
        .fetch_optional(pool)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

        // Verify password
        if !self.verify_password(&req.password, &user.password_hash)? {
            return Err(AppError::InvalidCredentials);
        }

        Ok(user)
    }

    /// Get user by ID
    pub async fn get_user_by_id(&self, pool: &PgPool, user_id: Uuid) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, username, email, password_hash, role, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(user)
    }
}

// Helper trait for pipeline operations
trait Pipe: Sized {
    fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        f(self)
    }
}

impl<T> Pipe for T {}
