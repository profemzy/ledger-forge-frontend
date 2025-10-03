use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::User;
use crate::services::AuthService;
use crate::utils::AppError;

/// Extension to inject authenticated user into request
#[allow(dead_code)]
#[derive(Clone)]
pub struct AuthUser(pub User);

/// Middleware to validate JWT and inject user into request
#[allow(dead_code)]
pub async fn auth_middleware(
    State(auth_service): State<AuthService>,
    State(pool): State<PgPool>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extract authorization header
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

    // Extract token from "Bearer <token>" format
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Invalid authorization format".to_string()))?;

    // Validate token and extract claims
    let token_data = auth_service.validate_token(token)?;
    let user_id = Uuid::parse_str(&token_data.claims.sub)
        .map_err(|_| AppError::InvalidToken)?;

    // Fetch user from database
    let user = auth_service.get_user_by_id(&pool, user_id).await?;

    // Inject user into request extensions
    request.extensions_mut().insert(AuthUser(user));

    Ok(next.run(request).await)
}

/// Extract authenticated user from request
#[allow(dead_code)]
pub async fn extract_auth_user(request: &Request) -> Result<User, AppError> {
    request
        .extensions()
        .get::<AuthUser>()
        .map(|auth_user| auth_user.0.clone())
        .ok_or_else(|| AppError::Unauthorized("No authenticated user found".to_string()))
}
