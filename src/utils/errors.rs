use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    // Authentication errors
    Unauthorized(String),
    InvalidCredentials,
    TokenExpired,
    InvalidToken,

    // Validation errors
    ValidationError(String),

    // Database errors
    DatabaseError(String),
    NotFound(String),
    Conflict(String),

    // Business logic errors (future features)
    #[allow(dead_code)]
    InsufficientBalance,
    #[allow(dead_code)]
    UnbalancedTransaction(String),
    #[allow(dead_code)]
    InvalidAccountType(String),

    // Generic errors
    InternalError(String),
    #[allow(dead_code)]
    BadRequest(String),
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::InvalidCredentials => write!(f, "Invalid credentials"),
            AppError::TokenExpired => write!(f, "Token has expired"),
            AppError::InvalidToken => write!(f, "Invalid token"),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::InsufficientBalance => write!(f, "Insufficient balance"),
            AppError::UnbalancedTransaction(msg) => write!(f, "Unbalanced transaction: {}", msg),
            AppError::InvalidAccountType(msg) => write!(f, "Invalid account type: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()),
            AppError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token has expired".to_string()),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", msg)),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::InsufficientBalance => (StatusCode::BAD_REQUEST, "Insufficient balance".to_string()),
            AppError::UnbalancedTransaction(msg) => (StatusCode::BAD_REQUEST, format!("Transaction must balance: {}", msg)),
            AppError::InvalidAccountType(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

// Convert sqlx errors to AppError
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            sqlx::Error::Database(db_err) => {
                // Check for unique constraint violations
                if let Some(constraint) = db_err.constraint() {
                    AppError::Conflict(format!("Duplicate entry: {}", constraint))
                } else {
                    AppError::DatabaseError(db_err.to_string())
                }
            }
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

// Convert validator errors
impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::ValidationError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
