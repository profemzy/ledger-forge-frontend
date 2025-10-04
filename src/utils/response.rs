use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

/// Standard success response wrapper
#[derive(Debug, Serialize, ToSchema)]
#[schema(example = json!({
    "success": true,
    "data": {},
    "message": null
}))]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data,
            message: None,
        }
    }

    #[allow(dead_code)]
    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            success: true,
            data,
            message: Some(message),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// Paginated response (for future pagination features)
#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub success: bool,
    pub data: Vec<T>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
}

impl<T: Serialize> PaginatedResponse<T> {
    #[allow(dead_code)]
    pub fn new(data: Vec<T>, page: i64, per_page: i64, total: i64) -> Self {
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Self {
            success: true,
            data,
            page,
            per_page,
            total,
            total_pages,
        }
    }
}

impl<T: Serialize> IntoResponse for PaginatedResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// Health check response
#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    #[schema(example = "ok")]
    pub status: String,
    #[schema(example = "0.1.0")]
    pub version: String,
    #[schema(example = "healthy")]
    pub database: String,
}

/// Empty response for successful operations with no data (future use)
#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct EmptyResponse {}

/// Helper function to create a success response
pub fn success<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse::success(data)
}

/// Helper function to create a success response with message (future use)
#[allow(dead_code)]
pub fn success_with_message<T: Serialize>(data: T, message: impl Into<String>) -> ApiResponse<T> {
    ApiResponse::success_with_message(data, message.into())
}

/// Helper to create created response (201)
pub fn created<T: Serialize>(data: T) -> impl IntoResponse {
    (StatusCode::CREATED, Json(ApiResponse::success(data)))
}

/// Helper to create no content response (204) - future use
#[allow(dead_code)]
pub fn no_content() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}
