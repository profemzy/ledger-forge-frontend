use uuid::Uuid;
use serde::Serialize;

use crate::api::client;
use crate::types::bills::{Bill, BillWithLineItems, CreateBillRequest, BillStatus};

#[derive(Serialize)]
struct ListQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")] vendor_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")] status: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] limit: Option<i64>,
}

fn status_to_str(s: &BillStatus) -> &'static str {
    match s { BillStatus::Open => "open", BillStatus::Paid => "paid", BillStatus::Partial => "partial", BillStatus::Void => "void" }
}

pub async fn list_bills(vendor_id: Option<Uuid>, status: Option<BillStatus>, limit: Option<i64>) -> Result<Vec<Bill>, String> {
    let q = ListQuery { vendor_id, status: status.as_ref().map(status_to_str), limit };
    let mut params = Vec::new();
    if let Some(id) = q.vendor_id { params.push(format!("vendor_id={}", id)); }
    if let Some(st) = q.status { params.push(format!("status={}", st)); }
    if let Some(l) = q.limit { params.push(format!("limit={}", l)); }
    let qs = if params.is_empty() { String::new() } else { format!("?{}", params.join("&")) };
    let path = format!("/bills{}", qs);
    client::get::<Vec<Bill>>(&path).await
}

pub async fn get_bill(id: Uuid) -> Result<BillWithLineItems, String> {
    let path = format!("/bills/{}", id);
    client::get::<BillWithLineItems>(&path).await
}

pub async fn create_bill(req: &CreateBillRequest) -> Result<Bill, String> {
    client::post::<_, Bill>("/bills", req).await
}

#[derive(Serialize)]
struct UpdateStatusBody<'a> { status: &'a str }

pub async fn update_bill_status(id: Uuid, status: BillStatus) -> Result<Bill, String> {
    let path = format!("/bills/{}/status", id);
    let body = UpdateStatusBody { status: status_to_str(&status) };
    client::put::<_, Bill>(&path, &body).await
}
