
use ledger_forge::models::{
    CreatePaymentRequest, PaymentApplicationRequest, CreateBillPaymentRequest,
    BillPaymentApplicationRequest, CreateInvoiceRequest, CreateInvoiceLineItemRequest,
    CreateContactRequest, CreateAccountRequest, AccountType, ContactType, InvoiceStatus,
};
use ledger_forge::services::{PaymentService, InvoiceService, ContactService, AccountService, CacheService};
use rust_decimal::Decimal;
use std::str::FromStr;
use chrono::NaiveDate;
use uuid::Uuid;

mod common;
use common::test_db::TestDb;

#[tokio::test]
async fn test_create_payment_with_applications() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    // Initialize services
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let payment_service = PaymentService::new_with_cache(cache_service.clone());
    let invoice_service = InvoiceService::new_with_cache(cache_service.clone());
    let contact_service = ContactService::new_with_cache(cache_service.clone());
    let account_service = AccountService::new_with_cache(cache_service);

    // Create a customer
    let customer_req = CreateContactRequest {
        contact_type: ContactType::Customer,
        name: "Test Customer".to_string(),
        email: Some("customer@test.com".to_string()),
        phone: Some("555-0100".to_string()),
        billing_address: Some("123 Test St".to_string()),
        shipping_address: None,
        company_id: None,
    };
    let customer = contact_service.create_contact(pool, customer_req).await.unwrap();

    // Create revenue account
    let revenue_account_req = CreateAccountRequest {
        code: "4000".to_string(),
        name: "Sales Revenue".to_string(),
        account_type: AccountType::Revenue,
        parent_account_id: None,
        company_id: None,
    };
    let revenue_account = account_service.create_account(pool, revenue_account_req).await.unwrap();

    // Create bank account
    let bank_account_req = CreateAccountRequest {
        code: "1000".to_string(),
        name: "Bank Account".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: None,
        company_id: None,
    };
    let bank_account = account_service.create_account(pool, bank_account_req).await.unwrap();

    // Create an invoice
    let invoice_req = CreateInvoiceRequest {
        invoice_number: "INV-001".to_string(),
        customer_id: customer.id,
        invoice_date: NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
        due_date: NaiveDate::from_ymd_opt(2024, 10, 31).unwrap(),
        ship_date: None,
        customer_memo: Some("Test invoice".to_string()),
        billing_address: Some("123 Test St".to_string()),
        shipping_address: None,
        company_id: None,
        line_items: vec![
            CreateInvoiceLineItemRequest {
                line_number: 1,
                item_description: "Test Item".to_string(),
                quantity: Decimal::from_str("2").unwrap(),
                unit_price: Decimal::from_str("500.00").unwrap(),
                discount_percent: None,
                tax_code: None,
                revenue_account_id: revenue_account.id,
            }
        ],
    };
    let invoice = invoice_service.create_invoice(pool, invoice_req).await.unwrap();

    // Create a payment with application
    let payment_req = CreatePaymentRequest {
        payment_number: Some("PAY-001".to_string()),
        customer_id: customer.id,
        payment_date: NaiveDate::from_ymd_opt(2024, 10, 15).unwrap(),
        amount: Decimal::from_str("1000.00").unwrap(),
        payment_method: "Check".to_string(),
        reference_number: Some("CHK-12345".to_string()),
        deposit_to_account_id: Some(bank_account.id),
        memo: Some("Payment for invoice INV-001".to_string()),
        company_id: None,
        applications: vec![
            PaymentApplicationRequest {
                invoice_id: invoice.invoice.id,
                amount_applied: Decimal::from_str("1000.00").unwrap(),
            }
        ],
    };

    let payment = payment_service.create_payment(pool, payment_req).await.unwrap();

    // Verify payment was created
    assert_eq!(payment.customer_id, customer.id);
    assert_eq!(payment.amount, Decimal::from_str("1000.00").unwrap());
    assert_eq!(payment.unapplied_amount, Some(Decimal::ZERO)); // Fully applied
    assert_eq!(payment.payment_method, "Check");

    // Verify invoice was updated
    let updated_invoice = invoice_service.get_invoice(pool, invoice.invoice.id).await.unwrap().unwrap();
    assert_eq!(updated_invoice.invoice.balance, Decimal::ZERO); // Fully paid
    assert_eq!(updated_invoice.invoice.status, InvoiceStatus::Paid);
}

#[tokio::test]
async fn test_create_payment_with_partial_application() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let payment_service = PaymentService::new_with_cache(cache_service.clone());
    let invoice_service = InvoiceService::new_with_cache(cache_service.clone());
    let contact_service = ContactService::new_with_cache(cache_service.clone());
    let account_service = AccountService::new_with_cache(cache_service);

    // Create customer and accounts
    let customer_req = CreateContactRequest {
        contact_type: ContactType::Customer,
        name: "Test Customer 2".to_string(),
        email: Some("customer2@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };
    let customer = contact_service.create_contact(pool, customer_req).await.unwrap();

    let revenue_account_req = CreateAccountRequest {
        code: "4001".to_string(),
        name: "Service Revenue".to_string(),
        account_type: AccountType::Revenue,
        parent_account_id: None,
        company_id: None,
    };
    let revenue_account = account_service.create_account(pool, revenue_account_req).await.unwrap();

    // Create invoice for $2000
    let invoice_req = CreateInvoiceRequest {
        invoice_number: "INV-002".to_string(),
        customer_id: customer.id,
        invoice_date: NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
        due_date: NaiveDate::from_ymd_opt(2024, 10, 31).unwrap(),
        ship_date: None,
        customer_memo: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
        line_items: vec![
            CreateInvoiceLineItemRequest {
                line_number: 1,
                item_description: "Service".to_string(),
                quantity: Decimal::from_str("1").unwrap(),
                unit_price: Decimal::from_str("2000.00").unwrap(),
                discount_percent: None,
                tax_code: None,
                revenue_account_id: revenue_account.id,
            }
        ],
    };
    let invoice = invoice_service.create_invoice(pool, invoice_req).await.unwrap();

    // Create partial payment of $1200
    let payment_req = CreatePaymentRequest {
        payment_number: Some("PAY-002".to_string()),
        customer_id: customer.id,
        payment_date: NaiveDate::from_ymd_opt(2024, 10, 15).unwrap(),
        amount: Decimal::from_str("1200.00").unwrap(),
        payment_method: "Credit Card".to_string(),
        reference_number: Some("CC-98765".to_string()),
        deposit_to_account_id: None,
        memo: None,
        company_id: None,
        applications: vec![
            PaymentApplicationRequest {
                invoice_id: invoice.invoice.id,
                amount_applied: Decimal::from_str("1200.00").unwrap(),
            }
        ],
    };

    let payment = payment_service.create_payment(pool, payment_req).await.unwrap();

    // Verify payment
    assert_eq!(payment.amount, Decimal::from_str("1200.00").unwrap());
    assert_eq!(payment.unapplied_amount, Some(Decimal::ZERO));

    // Verify invoice is partially paid
    let updated_invoice = invoice_service.get_invoice(pool, invoice.invoice.id).await.unwrap().unwrap();
    assert_eq!(updated_invoice.invoice.balance, Decimal::from_str("800.00").unwrap()); // $2000 - $1200
    assert_eq!(updated_invoice.invoice.status, InvoiceStatus::Partial);
}

#[tokio::test]
async fn test_create_payment_with_unapplied_amount() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let payment_service = PaymentService::new_with_cache(cache_service.clone());
    let contact_service = ContactService::new_with_cache(cache_service);

    // Create customer
    let customer_req = CreateContactRequest {
        contact_type: ContactType::Customer,
        name: "Test Customer 3".to_string(),
        email: Some("customer3@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };
    let customer = contact_service.create_contact(pool, customer_req).await.unwrap();

    // Create payment without applications (unapplied)
    let payment_req = CreatePaymentRequest {
        payment_number: Some("PAY-003".to_string()),
        customer_id: customer.id,
        payment_date: NaiveDate::from_ymd_opt(2024, 10, 20).unwrap(),
        amount: Decimal::from_str("500.00").unwrap(),
        payment_method: "Cash".to_string(),
        reference_number: None,
        deposit_to_account_id: None,
        memo: Some("Advance payment".to_string()),
        company_id: None,
        applications: vec![], // No applications
    };

    let payment = payment_service.create_payment(pool, payment_req).await.unwrap();

    // Verify payment has full unapplied amount
    assert_eq!(payment.amount, Decimal::from_str("500.00").unwrap());
    assert_eq!(payment.unapplied_amount, Some(Decimal::from_str("500.00").unwrap()));
}

#[tokio::test]
async fn test_list_unapplied_payments() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let payment_service = PaymentService::new_with_cache(cache_service.clone());
    let contact_service = ContactService::new_with_cache(cache_service);

    // Create customer
    let customer_req = CreateContactRequest {
        contact_type: ContactType::Customer,
        name: "Test Customer 4".to_string(),
        email: Some("customer4@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };
    let customer = contact_service.create_contact(pool, customer_req).await.unwrap();

    // Create unapplied payment
    let payment_req = CreatePaymentRequest {
        payment_number: Some("PAY-004".to_string()),
        customer_id: customer.id,
        payment_date: NaiveDate::from_ymd_opt(2024, 10, 25).unwrap(),
        amount: Decimal::from_str("750.00").unwrap(),
        payment_method: "Bank Transfer".to_string(),
        reference_number: Some("TRF-54321".to_string()),
        deposit_to_account_id: None,
        memo: None,
        company_id: None,
        applications: vec![],
    };

    payment_service.create_payment(pool, payment_req).await.unwrap();

    // List unapplied payments
    let unapplied = payment_service.list_unapplied_payments(pool, Some(customer.id)).await.unwrap();

    assert!(!unapplied.is_empty());
    assert!(unapplied.iter().all(|p| p.unapplied_amount.unwrap_or(Decimal::ZERO) > Decimal::ZERO));
}

#[tokio::test]
async fn test_apply_payment_to_invoices() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let payment_service = PaymentService::new_with_cache(cache_service.clone());
    let invoice_service = InvoiceService::new_with_cache(cache_service.clone());
    let contact_service = ContactService::new_with_cache(cache_service.clone());
    let account_service = AccountService::new_with_cache(cache_service);

    // Setup customer and accounts
    let customer_req = CreateContactRequest {
        contact_type: ContactType::Customer,
        name: "Test Customer 5".to_string(),
        email: Some("customer5@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };
    let customer = contact_service.create_contact(pool, customer_req).await.unwrap();

    let revenue_account_req = CreateAccountRequest {
        code: "4002".to_string(),
        name: "Consulting Revenue".to_string(),
        account_type: AccountType::Revenue,
        parent_account_id: None,
        company_id: None,
    };
    let revenue_account = account_service.create_account(pool, revenue_account_req).await.unwrap();

    // Create invoice
    let invoice_req = CreateInvoiceRequest {
        invoice_number: "INV-005".to_string(),
        customer_id: customer.id,
        invoice_date: NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
        due_date: NaiveDate::from_ymd_opt(2024, 10, 31).unwrap(),
        ship_date: None,
        customer_memo: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
        line_items: vec![
            CreateInvoiceLineItemRequest {
                line_number: 1,
                item_description: "Consulting Service".to_string(),
                quantity: Decimal::from_str("1").unwrap(),
                unit_price: Decimal::from_str("1500.00").unwrap(),
                discount_percent: None,
                tax_code: None,
                revenue_account_id: revenue_account.id,
            }
        ],
    };
    let invoice = invoice_service.create_invoice(pool, invoice_req).await.unwrap();

    // Create unapplied payment
    let payment_req = CreatePaymentRequest {
        payment_number: Some("PAY-005".to_string()),
        customer_id: customer.id,
        payment_date: NaiveDate::from_ymd_opt(2024, 10, 10).unwrap(),
        amount: Decimal::from_str("1500.00").unwrap(),
        payment_method: "Check".to_string(),
        reference_number: Some("CHK-99999".to_string()),
        deposit_to_account_id: None,
        memo: None,
        company_id: None,
        applications: vec![], // Create as unapplied
    };
    let payment = payment_service.create_payment(pool, payment_req).await.unwrap();

    // Verify payment is unapplied
    assert_eq!(payment.unapplied_amount, Some(Decimal::from_str("1500.00").unwrap()));

    // Now apply the payment to the invoice
    let applications = vec![
        PaymentApplicationRequest {
            invoice_id: invoice.invoice.id,
            amount_applied: Decimal::from_str("1500.00").unwrap(),
        }
    ];

    payment_service.apply_payment_to_invoices(pool, payment.id, applications).await.unwrap();

    // Verify payment is now fully applied
    let updated_payment = payment_service.get_payment_by_id(pool, payment.id).await.unwrap().unwrap();
    assert_eq!(updated_payment.unapplied_amount, Some(Decimal::ZERO));

    // Verify invoice is paid
    let updated_invoice = invoice_service.get_invoice(pool, invoice.invoice.id).await.unwrap().unwrap();
    assert_eq!(updated_invoice.invoice.balance, Decimal::ZERO);
    assert_eq!(updated_invoice.invoice.status, InvoiceStatus::Paid);
}

#[tokio::test]
async fn test_get_invoice_payments() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let payment_service = PaymentService::new_with_cache(cache_service.clone());
    let invoice_service = InvoiceService::new_with_cache(cache_service.clone());
    let contact_service = ContactService::new_with_cache(cache_service.clone());
    let account_service = AccountService::new_with_cache(cache_service);

    // Setup
    let customer_req = CreateContactRequest {
        contact_type: ContactType::Customer,
        name: "Test Customer 6".to_string(),
        email: Some("customer6@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };
    let customer = contact_service.create_contact(pool, customer_req).await.unwrap();

    let revenue_account_req = CreateAccountRequest {
        code: "4003".to_string(),
        name: "Product Revenue".to_string(),
        account_type: AccountType::Revenue,
        parent_account_id: None,
        company_id: None,
    };
    let revenue_account = account_service.create_account(pool, revenue_account_req).await.unwrap();

    // Create invoice
    let invoice_req = CreateInvoiceRequest {
        invoice_number: "INV-006".to_string(),
        customer_id: customer.id,
        invoice_date: NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
        due_date: NaiveDate::from_ymd_opt(2024, 10, 31).unwrap(),
        ship_date: None,
        customer_memo: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
        line_items: vec![
            CreateInvoiceLineItemRequest {
                line_number: 1,
                item_description: "Product".to_string(),
                quantity: Decimal::from_str("1").unwrap(),
                unit_price: Decimal::from_str("3000.00").unwrap(),
                discount_percent: None,
                tax_code: None,
                revenue_account_id: revenue_account.id,
            }
        ],
    };
    let invoice = invoice_service.create_invoice(pool, invoice_req).await.unwrap();

    // Create two payments for the invoice
    let payment1_req = CreatePaymentRequest {
        payment_number: Some("PAY-006-1".to_string()),
        customer_id: customer.id,
        payment_date: NaiveDate::from_ymd_opt(2024, 10, 15).unwrap(),
        amount: Decimal::from_str("1500.00").unwrap(),
        payment_method: "Check".to_string(),
        reference_number: Some("CHK-111".to_string()),
        deposit_to_account_id: None,
        memo: None,
        company_id: None,
        applications: vec![
            PaymentApplicationRequest {
                invoice_id: invoice.invoice.id,
                amount_applied: Decimal::from_str("1500.00").unwrap(),
            }
        ],
    };
    payment_service.create_payment(pool, payment1_req).await.unwrap();

    let payment2_req = CreatePaymentRequest {
        payment_number: Some("PAY-006-2".to_string()),
        customer_id: customer.id,
        payment_date: NaiveDate::from_ymd_opt(2024, 10, 20).unwrap(),
        amount: Decimal::from_str("1500.00").unwrap(),
        payment_method: "Credit Card".to_string(),
        reference_number: Some("CC-222".to_string()),
        deposit_to_account_id: None,
        memo: None,
        company_id: None,
        applications: vec![
            PaymentApplicationRequest {
                invoice_id: invoice.invoice.id,
                amount_applied: Decimal::from_str("1500.00").unwrap(),
            }
        ],
    };
    payment_service.create_payment(pool, payment2_req).await.unwrap();

    // Get payments for the invoice
    let invoice_payments = payment_service.get_invoice_payments(pool, invoice.invoice.id).await.unwrap();

    // Verify we got both payments
    assert_eq!(invoice_payments.len(), 2);
    
    // Verify total applied equals invoice amount
    let total_applied: Decimal = invoice_payments.iter()
        .map(|p| p.amount)
        .sum();
    assert_eq!(total_applied, Decimal::from_str("3000.00").unwrap());
}

#[tokio::test]
async fn test_create_bill_payment() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let payment_service = PaymentService::new_with_cache(cache_service.clone());
    let contact_service = ContactService::new_with_cache(cache_service);

    // Create vendor
    let vendor_req = CreateContactRequest {
        contact_type: ContactType::Vendor,
        name: "Test Vendor".to_string(),
        email: Some("vendor@test.com".to_string()),
        phone: Some("555-0200".to_string()),
        billing_address: Some("456 Vendor St".to_string()),
        shipping_address: None,
        company_id: None,
    };
    let vendor = contact_service.create_contact(pool, vendor_req).await.unwrap();

    // Create bill payment (without bill applications for now)
    let bill_payment_req = CreateBillPaymentRequest {
        payment_number: Some("BP-001".to_string()),
        vendor_id: vendor.id,
        payment_date: NaiveDate::from_ymd_opt(2024, 10, 15).unwrap(),
        amount: Decimal::from_str("2500.00").unwrap(),
        payment_method: "Check".to_string(),
        reference_number: Some("CHK-VENDOR-001".to_string()),
        bank_account_id: None,
        memo: Some("Payment for supplies".to_string()),
        company_id: None,
        applications: vec![],
    };

    let bill_payment = payment_service.create_bill_payment(pool, bill_payment_req).await.unwrap();

    // Verify bill payment was created
    assert_eq!(bill_payment.vendor_id, vendor.id);
    assert_eq!(bill_payment.amount, Decimal::from_str("2500.00").unwrap());
    assert_eq!(bill_payment.payment_method, "Check");
    assert_eq!(bill_payment.reference_number, Some("CHK-VENDOR-001".to_string()));
}

#[tokio::test]
async fn test_payment_validation_exceeds_amount() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let payment_service = PaymentService::new_with_cache(cache_service.clone());
    let invoice_service = InvoiceService::new_with_cache(cache_service.clone());
    let contact_service = ContactService::new_with_cache(cache_service.clone());
    let account_service = AccountService::new_with_cache(cache_service);

    // Setup
    let customer_req = CreateContactRequest {
        contact_type: ContactType::Customer,
        name: "Test Customer 7".to_string(),
        email: Some("customer7@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };
    let customer = contact_service.create_contact(pool, customer_req).await.unwrap();

    let revenue_account_req = CreateAccountRequest {
        code: "4004".to_string(),
        name: "Other Revenue".to_string(),
        account_type: AccountType::Revenue,
        parent_account_id: None,
        company_id: None,
    };
    let revenue_account = account_service.create_account(pool, revenue_account_req).await.unwrap();

    // Create invoice
    let invoice_req = CreateInvoiceRequest {
        invoice_number: "INV-007".to_string(),
        customer_id: customer.id,
        invoice_date: NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
        due_date: NaiveDate::from_ymd_opt(2024, 10, 31).unwrap(),
        ship_date: None,
        customer_memo: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
        line_items: vec![
            CreateInvoiceLineItemRequest {
                line_number: 1,
                item_description: "Service".to_string(),
                quantity: Decimal::from_str("1").unwrap(),
                unit_price: Decimal::from_str("1000.00").unwrap(),
                discount_percent: None,
                tax_code: None,
                revenue_account_id: revenue_account.id,
            }
        ],
    };
    let invoice = invoice_service.create_invoice(pool, invoice_req).await.unwrap();

    // Try to create payment with application exceeding payment amount
    let payment_req = CreatePaymentRequest {
        payment_number: Some("PAY-007".to_string()),
        customer_id: customer.id,
        payment_date: NaiveDate::from_ymd_opt(2024, 10, 15).unwrap(),
        amount: Decimal::from_str("500.00").unwrap(), // Payment is $500
        payment_method: "Check".to_string(),
        reference_number: None,
        deposit_to_account_id: None,
        memo: None,
        company_id: None,
        applications: vec![
            PaymentApplicationRequest {
                invoice_id: invoice.invoice.id,
                amount_applied: Decimal::from_str("1000.00").unwrap(), // Trying to apply $1000
            }
        ],
    };

    // This should fail
    let result = payment_service.create_payment(pool, payment_req).await;
    assert!(result.is_err());
}