use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type, PartialEq)]
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

#[derive(Debug, Deserialize, Validate)]
pub struct CreateContactRequest {
    pub contact_type: ContactType,

    #[validate(length(min = 1, max = 255))]
    pub name: String,

    #[validate(email)]
    pub email: Option<String>,

    pub phone: Option<String>,
    pub billing_address: Option<String>,
    pub shipping_address: Option<String>,
    pub company_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateContactRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,

    #[validate(email)]
    pub email: Option<String>,

    pub phone: Option<String>,
    pub billing_address: Option<String>,
    pub shipping_address: Option<String>,
}
