use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::{Contact, ContactType, CreateContactRequest, UpdateContactRequest};
use crate::routes::AppState;
use crate::utils::{created, no_content, success, ApiResponse, AppError, Result};

/// Query parameters for listing contacts
#[derive(Debug, Deserialize)]
pub struct ListContactsQuery {
    #[serde(default)]
    pub contact_type: Option<String>,
    #[serde(default)]
    pub company_id: Option<Uuid>,
    #[serde(default)]
    pub limit: Option<i64>,
}

/// List all contacts
#[utoipa::path(
    get,
    path = "/api/v1/contacts",
    tag = "contacts",
    params(
        ("contact_type" = Option<String>, Query, description = "Filter by contact type (Customer, Vendor, Employee)"),
        ("company_id" = Option<String>, Query, description = "Filter by company ID"),
        ("limit" = Option<i64>, Query, description = "Limit number of results")
    ),
    responses(
        (status = 200, description = "List of contacts", body = ApiResponse<Vec<Contact>>),
        (status = 400, description = "Invalid query parameters")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_contacts(
    State(state): State<AppState>,
    Query(params): Query<ListContactsQuery>,
) -> Result<impl axum::response::IntoResponse> {
    // Parse contact type if provided
    let contact_type = if let Some(type_str) = params.contact_type {
        Some(parse_contact_type(&type_str)?)
    } else {
        None
    };

    let contacts = state
        .contact_service
        .list_contacts(
            &state.pool,
            contact_type,
            params.company_id,
            params.limit,
        )
        .await?;

    Ok(success(contacts))
}

/// Create a new contact
#[utoipa::path(
    post,
    path = "/api/v1/contacts",
    tag = "contacts",
    request_body = CreateContactRequest,
    responses(
        (status = 201, description = "Contact created successfully", body = ApiResponse<Contact>),
        (status = 400, description = "Invalid request data")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_contact(
    State(state): State<AppState>,
    Json(req): Json<CreateContactRequest>,
) -> Result<impl axum::response::IntoResponse> {
    let contact = state.contact_service.create_contact(&state.pool, req).await?;
    Ok(created(contact))
}

/// Get contact by ID
#[utoipa::path(
    get,
    path = "/api/v1/contacts/{id}",
    tag = "contacts",
    params(
        ("id" = Uuid, Path, description = "Contact ID")
    ),
    responses(
        (status = 200, description = "Contact details", body = ApiResponse<Contact>),
        (status = 404, description = "Contact not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_contact(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    let contact = state.contact_service.get_contact_by_id(&state.pool, id).await?;
    Ok(success(contact))
}

/// Update contact
#[utoipa::path(
    put,
    path = "/api/v1/contacts/{id}",
    tag = "contacts",
    params(
        ("id" = Uuid, Path, description = "Contact ID")
    ),
    request_body = UpdateContactRequest,
    responses(
        (status = 200, description = "Contact updated successfully", body = ApiResponse<Contact>),
        (status = 400, description = "Invalid request data"),
        (status = 404, description = "Contact not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_contact(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateContactRequest>,
) -> Result<impl axum::response::IntoResponse> {
    let contact = state
        .contact_service
        .update_contact(&state.pool, id, req)
        .await?;
    Ok(success(contact))
}

/// Delete contact
#[utoipa::path(
    delete,
    path = "/api/v1/contacts/{id}",
    tag = "contacts",
    params(
        ("id" = Uuid, Path, description = "Contact ID")
    ),
    responses(
        (status = 204, description = "Contact deleted successfully"),
        (status = 404, description = "Contact not found"),
        (status = 409, description = "Contact has existing transactions")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_contact(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    state.contact_service.delete_contact(&state.pool, id).await?;
    Ok(no_content())
}

/// Get all customers
#[utoipa::path(
    get,
    path = "/api/v1/contacts/customers",
    tag = "contacts",
    responses(
        (status = 200, description = "List of customers", body = ApiResponse<Vec<Contact>>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_customers(
    State(state): State<AppState>,
) -> Result<impl axum::response::IntoResponse> {
    let customers = state.contact_service.get_customers(&state.pool).await?;
    Ok(success(customers))
}

/// Get all vendors
#[utoipa::path(
    get,
    path = "/api/v1/contacts/vendors",
    tag = "contacts",
    responses(
        (status = 200, description = "List of vendors", body = ApiResponse<Vec<Contact>>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_vendors(
    State(state): State<AppState>,
) -> Result<impl axum::response::IntoResponse> {
    let vendors = state.contact_service.get_vendors(&state.pool).await?;
    Ok(success(vendors))
}

/// Get all employees
#[utoipa::path(
    get,
    path = "/api/v1/contacts/employees",
    tag = "contacts",
    responses(
        (status = 200, description = "List of employees", body = ApiResponse<Vec<Contact>>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_employees(
    State(state): State<AppState>,
) -> Result<impl axum::response::IntoResponse> {
    let employees = state.contact_service.get_employees(&state.pool).await?;
    Ok(success(employees))
}

// Helper function to parse contact type from string
fn parse_contact_type(type_str: &str) -> Result<ContactType> {
    match type_str.to_lowercase().as_str() {
        "customer" => Ok(ContactType::Customer),
        "vendor" => Ok(ContactType::Vendor),
        "employee" => Ok(ContactType::Employee),
        _ => Err(AppError::BadRequest(format!(
            "Invalid contact type: {}. Must be one of: Customer, Vendor, Employee",
            type_str
        ))),
    }
}
