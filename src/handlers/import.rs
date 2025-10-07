use axum::{
    extract::State,
    http::StatusCode,
    body::Bytes,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::routes::AppState;
use crate::utils::{success, AppError, Result, ImportResult};

/// Import result response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ImportResultResponse {
    pub total_rows: usize,
    pub successful: usize,
    pub failed: usize,
    pub errors: Vec<ImportErrorResponse>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ImportErrorResponse {
    pub row_number: usize,
    pub code: String,
    pub error_message: String,
}

impl From<ImportResult> for ImportResultResponse {
    fn from(result: ImportResult) -> Self {
        Self {
            total_rows: result.total_rows,
            successful: result.successful,
            failed: result.failed,
            errors: result.errors.into_iter().map(|e| ImportErrorResponse {
                row_number: e.row_number,
                code: e.code,
                error_message: e.error_message,
            }).collect(),
        }
    }
}

/// Import Chart of Accounts from CSV
#[utoipa::path(
    post,
    path = "/api/v1/import/accounts",
    tag = "import",
    request_body(content = Vec<u8>, description = "CSV file content", content_type = "text/csv"),
    responses(
        (status = 200, description = "Import completed", body = ImportResultResponse),
        (status = 400, description = "Invalid CSV format"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn import_accounts_csv(
    State(state): State<AppState>,
    body: Bytes,
) -> Result<impl axum::response::IntoResponse> {
    // Import accounts
    let result = state.import_service
        .import_accounts_from_csv(&state.pool, &body)
        .await?;

    let response = ImportResultResponse::from(result);

    // Return appropriate status based on results
    if response.failed > 0 && response.successful == 0 {
        // All failed
        return Err(AppError::ValidationError(
            format!("Import failed: {} errors", response.failed)
        ));
    }

    Ok(success(response))
}

/// Get CSV template for Chart of Accounts
#[utoipa::path(
    get,
    path = "/api/v1/import/accounts/template",
    tag = "import",
    responses(
        (status = 200, description = "CSV template", content_type = "text/csv"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_accounts_csv_template(
    State(_state): State<AppState>,
) -> Result<(StatusCode, [(String, String); 2], String)> {
    let template = crate::services::ImportService::get_accounts_csv_template();
    
    Ok((
        StatusCode::OK,
        [
            ("Content-Type".to_string(), "text/csv".to_string()),
            ("Content-Disposition".to_string(), "attachment; filename=\"chart_of_accounts_template.csv\"".to_string()),
        ],
        template,
    ))
}
