use axum::{extract::State, Json};

#[allow(unused_imports)]
use crate::models::{AuthResponse, CreateUserRequest, LoginRequest, User, UserResponse};
use crate::routes::AppState;
use crate::utils::{created, success, ApiResponse, AppError, Result};

/// Register a new user
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "auth",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User registered successfully", body = ApiResponse<AuthResponse>),
        (status = 400, description = "Invalid request data"),
        (status = 409, description = "User already exists")
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<impl axum::response::IntoResponse> {
    // Register user
    let user = state.auth_service.register(&state.pool, req).await?;

    // Generate tokens
    let access_token = state.auth_service.generate_access_token(&user)?;
    let refresh_token = state.auth_service.generate_refresh_token(&user)?;

    // Create response
    let response = AuthResponse {
        access_token,
        refresh_token,
        user: user.into(),
    };

    Ok(created(response))
}

/// Login user
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<AuthResponse>),
        (status = 401, description = "Invalid credentials"),
        (status = 404, description = "User not found")
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<ApiResponse<AuthResponse>> {
    // Authenticate user
    let user = state.auth_service.login(&state.pool, req).await?;

    // Generate tokens
    let access_token = state.auth_service.generate_access_token(&user)?;
    let refresh_token = state.auth_service.generate_refresh_token(&user)?;

    // Create response
    let response = AuthResponse {
        access_token,
        refresh_token,
        user: user.into(),
    };

    Ok(success(response))
}

/// Refresh access token using refresh token
#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    tag = "auth",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = ApiResponse<TokenResponse>),
        (status = 401, description = "Invalid or expired refresh token")
    )
)]
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<ApiResponse<TokenResponse>> {
    // Validate refresh token
    let token_data = state.auth_service.validate_token(&payload.refresh_token)?;

    // Get user from token claims
    let user_id = uuid::Uuid::parse_str(&token_data.claims.sub)
        .map_err(|_| AppError::InvalidToken)?;

    let user = state.auth_service.get_user_by_id(&state.pool, user_id).await?;

    // Generate new access token
    let access_token = state.auth_service.generate_access_token(&user)?;

    let response = TokenResponse { access_token };

    Ok(success(response))
}

/// Get current user profile (requires authentication)
#[utoipa::path(
    get,
    path = "/api/v1/auth/me",
    tag = "auth",
    responses(
        (status = 200, description = "Current user profile retrieved", body = ApiResponse<UserResponse>),
        (status = 401, description = "Unauthorized - missing or invalid token")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn me(
    State(state): State<AppState>,
    request: axum::http::Request<axum::body::Body>,
) -> Result<ApiResponse<UserResponse>> {
    // Extract authorization header
    let auth_header = request
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

    // Extract token from "Bearer <token>" format
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Invalid authorization format".to_string()))?;

    // Validate token and extract claims
    let token_data = state.auth_service.validate_token(token)?;
    let user_id = uuid::Uuid::parse_str(&token_data.claims.sub)
        .map_err(|_| AppError::InvalidToken)?;

    // Fetch user from database
    let user = state.auth_service.get_user_by_id(&state.pool, user_id).await?;

    Ok(success(user.into()))
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
}
