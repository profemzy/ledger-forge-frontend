use serde::Serialize;
use uuid::Uuid;

use crate::api::client;
use crate::types::invoices::{
    Invoice, InvoiceWithLineItems, InvoiceStatus, CreateInvoiceRequest,
};
use crate::types::payments::Payment;

fn status_to_str(s: &InvoiceStatus) -> &'static str {
    match s {
        InvoiceStatus::Draft => "draft",
        InvoiceStatus::Sent => "sent",
        InvoiceStatus::Paid => "paid",
        InvoiceStatus::Partial => "partial",
        InvoiceStatus::Overdue => "overdue",
        InvoiceStatus::Void => "void",
    }
}

pub async fn list_invoices(
    customer_id: Option<Uuid>,
    status: Option<InvoiceStatus>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Invoice>, String> {
    let mut params = Vec::new();
    if let Some(cid) = customer_id { params.push(format!("customer_id={}", cid)); }
    if let Some(s) = status { params.push(format!("status={}", status_to_str(&s))); }
    if let Some(l) = limit { params.push(format!("limit={}", l)); }
    if let Some(o) = offset { params.push(format!("offset={}", o)); }
    let qs = if params.is_empty() { String::new() } else { format!("?{}", params.join("&")) };
    let path = format!("/invoices{}", qs);
    client::get::<Vec<Invoice>>(&path).await
}

pub async fn get_invoice(id: Uuid) -> Result<InvoiceWithLineItems, String> {
    let path = format!("/invoices/{}", id);
    client::get::<InvoiceWithLineItems>(&path).await
}

pub async fn create_invoice(req: &CreateInvoiceRequest) -> Result<InvoiceWithLineItems, String> {
    client::post::<_, InvoiceWithLineItems>("/invoices", req).await
}

#[derive(Serialize)]
struct UpdateStatusBody<'a> { status: &'a str }

pub async fn update_invoice_status(id: Uuid, status: InvoiceStatus) -> Result<Invoice, String> {
    let path = format!("/invoices/{}/status", id);
    let body = UpdateStatusBody { status: status_to_str(&status) };
    client::put::<_, Invoice>(&path, &body).await
}

pub async fn get_overdue_invoices() -> Result<Vec<Invoice>, String> {
    client::get::<Vec<Invoice>>("/invoices/overdue").await
}

pub async fn get_customer_invoices(customer_id: Uuid) -> Result<Vec<Invoice>, String> {
    let path = format!("/customers/{}/invoices", customer_id);
    client::get::<Vec<Invoice>>(&path).await
}

pub async fn get_invoice_payments(invoice_id: Uuid) -> Result<Vec<Payment>, String> {
    let path = format!("/invoices/{}/payments", invoice_id);
    client::get::<Vec<Payment>>(&path).await
}
