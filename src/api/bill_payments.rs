use crate::api::client;
use crate::types::bill_payments::{BillPayment, CreateBillPaymentRequest};

pub async fn create_bill_payment(req: &CreateBillPaymentRequest) -> Result<BillPayment, String> {
    client::post::<_, BillPayment>("/bill-payments", req).await
}
