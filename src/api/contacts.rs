use crate::api::client;
use crate::types::contacts::Contact;

pub async fn list_contacts() -> Result<Vec<Contact>, String> {
    client::get::<Vec<Contact>>("/contacts").await
}

pub async fn list_customers() -> Result<Vec<Contact>, String> {
    client::get::<Vec<Contact>>("/contacts/customers").await
}

#[allow(dead_code)]
pub async fn list_vendors() -> Result<Vec<Contact>, String> {
    client::get::<Vec<Contact>>("/contacts/vendors").await
}
