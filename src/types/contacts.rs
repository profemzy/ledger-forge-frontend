use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ContactType {
    Customer,
    Vendor,
    Employee,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    pub id: Uuid,
    pub contact_type: ContactType,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub billing_address: Option<String>,
    pub company_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

