#![allow(dead_code)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, ToSchema)]
pub struct Contact {
    pub id: Uuid,
    pub contact_type: ContactType,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub billing_address: Option<String>,
    pub shipping_address: Option<String>,
    pub company_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type, PartialEq, ToSchema)]
#[sqlx(type_name = "varchar", rename_all = "PascalCase")]
pub enum ContactType {
    Customer,
    Vendor,
    Employee,
}

impl std::fmt::Display for ContactType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContactType::Customer => write!(f, "Customer"),
            ContactType::Vendor => write!(f, "Vendor"),
            ContactType::Employee => write!(f, "Employee"),
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateContactRequest {
    #[schema(example = "Customer")]
    pub contact_type: ContactType,

    #[validate(length(min = 1, max = 255))]
    #[schema(example = "Acme Corporation")]
    pub name: String,

    #[validate(email)]
    #[schema(example = "contact@acme.com")]
    pub email: Option<String>,

    #[schema(example = "+1-555-0123")]
    pub phone: Option<String>,

    #[schema(example = "123 Main St, New York, NY 10001")]
    pub billing_address: Option<String>,

    #[schema(example = "456 Oak Ave, Los Angeles, CA 90001")]
    pub shipping_address: Option<String>,

    pub company_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateContactRequest {
    #[validate(length(min = 1, max = 255))]
    #[schema(example = "Acme Corporation LLC")]
    pub name: Option<String>,

    #[validate(email)]
    #[schema(example = "newcontact@acme.com")]
    pub email: Option<String>,

    #[schema(example = "+1-555-9999")]
    pub phone: Option<String>,

    #[schema(example = "789 Pine Rd, Boston, MA 02101")]
    pub billing_address: Option<String>,

    #[schema(example = "321 Elm St, Chicago, IL 60601")]
    pub shipping_address: Option<String>,
}
