use serde::Serialize;
use uuid::Uuid;

use crate::api::client;
use crate::types::payments::{Payment, CreatePaymentRequest, PaymentApplicationRequest};

pub async fn list_payments() -> Result<Vec<Payment>, String> {
    client::get::<Vec<Payment>>("/payments").await
}

pub async fn get_payment(id: Uuid) -> Result<Payment, String> {
    let path = format!("/payments/{}", id);
    client::get::<Payment>(&path).await
}

pub async fn create_payment(req: &CreatePaymentRequest) -> Result<Payment, String> {
    client::post::<_, Payment>("/payments", req).await
}

#[derive(Serialize)]
struct ApplyBody { applications: Vec<PaymentApplicationRequest> }

pub async fn apply_payment(id: Uuid, apps: Vec<PaymentApplicationRequest>) -> Result<Payment, String> {
    let path = format!("/payments/{}/apply", id);
    let body = ApplyBody { applications: apps };
    client::put::<_, Payment>(&path, &body).await
}

pub async fn get_unapplied_payments() -> Result<Vec<Payment>, String> {
    client::get::<Vec<Payment>>("/payments/unapplied").await
}

