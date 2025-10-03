use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Item {
    pub id: Uuid,
    pub quickbooks_id: Option<String>,
    pub name: String,
    pub sku: Option<String>,
    pub item_type: ItemType,
    pub description: Option<String>,
    pub unit_price: Option<Decimal>,
    pub purchase_cost: Option<Decimal>,
    pub quantity_on_hand: Option<Decimal>,
    pub income_account_id: Option<Uuid>,
    pub expense_account_id: Option<Uuid>,
    pub asset_account_id: Option<Uuid>,
    pub active: Option<bool>,
    pub taxable: Option<bool>,
    pub company_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type, PartialEq)]
#[sqlx(type_name = "varchar", rename_all = "PascalCase")]
pub enum ItemType {
    Service,
    Inventory,
    #[serde(rename = "Non-Inventory")]
    #[sqlx(rename = "Non-Inventory")]
    NonInventory,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateItemRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub sku: Option<String>,
    pub item_type: ItemType,
    pub description: Option<String>,
    #[validate(range(min = 0))]
    pub unit_price: Option<Decimal>,
    #[validate(range(min = 0))]
    pub purchase_cost: Option<Decimal>,
    pub income_account_id: Option<Uuid>,
    pub expense_account_id: Option<Uuid>,
    pub asset_account_id: Option<Uuid>,
    pub taxable: Option<bool>,
    pub company_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateItemRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub sku: Option<String>,
    pub description: Option<String>,
    #[validate(range(min = 0))]
    pub unit_price: Option<Decimal>,
    #[validate(range(min = 0))]
    pub purchase_cost: Option<Decimal>,
    pub active: Option<bool>,
    pub taxable: Option<bool>,
}
