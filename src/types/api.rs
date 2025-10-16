// Placeholder API envelope types

#[derive(Debug, serde::Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
    pub message: Option<String>,
}

