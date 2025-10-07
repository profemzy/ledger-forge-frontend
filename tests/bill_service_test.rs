
use ledger_forge::models::{
    CreateBillRequest, CreateBillLineItemRequest, BillStatus,
    CreateContactRequest, CreateAccountRequest, AccountType, ContactType,
};
use ledger_forge::services::{BillService, ContactService, AccountService, CacheService};
use rust_decimal::Decimal;
use std::str::FromStr;
use chrono::NaiveDate;

mod common;
use common::test_db::TestDb;

#[tokio::test]
async fn test_create_bill_with_line_items() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    // Initialize services
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let bill_service = BillService::new_with_cache(cache_service.clone());
    let contact_service = ContactService::new_with_cache(cache_service.clone());
    let account_service = AccountService::new_with_cache(cache_service);

    // Create a vendor
    let vendor_req = CreateContactRequest {
        contact_type: ContactType::Vendor,
        name: "Office Supplies Co".to_string(),
        email: Some("vendor@supplies.com".to_string()),
        phone: Some("555-0100".to_string()),
        billing_address: Some("123 Vendor St".to_string()),
        shipping_address: None,
        company_id: None,
    };
    let vendor = contact_service.create_contact(pool, vendor_req).await.unwrap();

    // Create expense account
    let expense_account_req = CreateAccountRequest {
        code: "5000".to_string(),
        name: "Office Supplies Expense".to_string(),
        account_type: AccountType::Expense,
        parent_account_id: None,
        company_id: None,
    };
    let expense_account = account_service.create_account(pool, expense_account_req).await.unwrap();

    // Create a bill
    let bill_req = CreateBillRequest {
        bill_number: Some("BILL-001".to_string()),
        vendor_id: vendor.id,
        bill_date: NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
        due_date: NaiveDate::from_ymd_opt(2024, 10, 31).unwrap(),
        memo: Some("Office supplies purchase".to_string()),
        company_id: None,
        line_items: vec![
            CreateBillLineItemRequest {
                line_number: 1,
                description: Some("Printer paper".to_string()),
                amount: Decimal::from_str("150.00").unwrap(),
                expense_account_id: expense_account.id,
                billable: Some(false),
                customer_id: None,
            },
            CreateBillLineItemRequest {
                line_number: 2,
                description: Some("Pens and pencils".to_string()),
                amount: Decimal::from_str("75.50").unwrap(),
                expense_account_id: expense_account.id,
                billable: Some(false),
                customer_id: None,
            }
        ],
    };

    let bill = bill_service.create_bill(pool, bill_req).await.unwrap();

    // Verify bill was created
    assert_eq!(bill.vendor_id, vendor.id);
    assert_eq!(bill.total_amount, Decimal::from_str("225.50").unwrap());
    assert_eq!(bill.balance, Decimal::from_str("225.50").unwrap());
    assert_eq!(bill.status, BillStatus::Open);
    assert_eq!(bill.bill_number, Some("BILL-001".to_string()));
}

#[tokio::test]
async fn test_get_bill_by_id() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let bill_service = BillService::new_with_cache(cache_service.clone());
    let contact_service = ContactService::new_with_cache(cache_service.clone());
    let account_service = AccountService::new_with_cache(cache_service);

    // Setup vendor and account
    let vendor_req = CreateContactRequest {
        contact_type: ContactType::Vendor,
        name: "Test Vendor 2".to_string(),
        email: Some("vendor2@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };
    let vendor = contact_service.create_contact(pool, vendor_req).await.unwrap();

    let expense_account_req = CreateAccountRequest {
        code: "5001".to_string(),
        name: "Utilities Expense".to_string(),
        account_type: AccountType::Expense,
        parent_account_id: None,
        company_id: None,
    };
    let expense_account = account_service.create_account(pool, expense_account_req).await.unwrap();

    // Create bill
    let bill_req = CreateBillRequest {
        bill_number: Some("BILL-002".to_string()),
        vendor_id: vendor.id,
        bill_date: NaiveDate::from_ymd_opt(2024, 10, 5).unwrap(),
        due_date: NaiveDate::from_ymd_opt(2024, 11, 5).unwrap(),
        memo: None,
        company_id: None,
        line_items: vec![
            CreateBillLineItemRequest {
                line_number: 1,
                description: Some("Electricity".to_string()),
                amount: Decimal::from_str("450.00").unwrap(),
                expense_account_id: expense_account.id,
                billable: Some(false),
                customer_id: None,
            }
        ],
    };
    let bill = bill_service.create_bill(pool, bill_req).await.unwrap();

    // Get bill by ID
    let retrieved_bill = bill_service.get_bill_by_id(pool, bill.id).await.unwrap().unwrap();

    // Verify bill and line items
    assert_eq!(retrieved_bill.bill.id, bill.id);
    assert_eq!(retrieved_bill.line_items.len(), 1);
    assert_eq!(retrieved_bill.line_items[0].description, Some("Electricity".to_string()));
    assert_eq!(retrieved_bill.line_items[0].amount, Decimal::from_str("450.00").unwrap());
}

#[tokio::test]
async fn test_list_bills() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let bill_service = BillService::new_with_cache(cache_service.clone());
    let contact_service = ContactService::new_with_cache(cache_service.clone());
    let account_service = AccountService::new_with_cache(cache_service);

    // Setup
    let vendor_req = CreateContactRequest {
        contact_type: ContactType::Vendor,
        name: "Test Vendor 3".to_string(),
        email: Some("vendor3@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };
    let vendor = contact_service.create_contact(pool, vendor_req).await.unwrap();

    let expense_account_req = CreateAccountRequest {
        code: "5002".to_string(),
        name: "Rent Expense".to_string(),
        account_type: AccountType::Expense,
        parent_account_id: None,
        company_id: None,
    };
    let expense_account = account_service.create_account(pool, expense_account_req).await.unwrap();

    // Create multiple bills
    for i in 1..=3 {
        let bill_req = CreateBillRequest {
            bill_number: Some(format!("BILL-00{}", i)),
            vendor_id: vendor.id,
            bill_date: NaiveDate::from_ymd_opt(2024, 10, i as u32).unwrap(),
            due_date: NaiveDate::from_ymd_opt(2024, 10, (i + 30) as u32).unwrap(),
            memo: None,
            company_id: None,
            line_items: vec![
                CreateBillLineItemRequest {
                    line_number: 1,
                    description: Some(format!("Expense {}", i)),
                    amount: Decimal::from_str(&format!("{}.00", i * 100)).unwrap(),
                    expense_account_id: expense_account.id,
                    billable: Some(false),
                    customer_id: None,
                }
            ],
        };
        bill_service.create_bill(pool, bill_req).await.unwrap();
    }

    // List all bills
    let bills = bill_service.list_bills(pool, None, None, None, None).await.unwrap();
    assert!(bills.len() >= 3);

    // List bills for specific vendor
    let vendor_bills = bill_service.list_bills(pool, Some(vendor.id), None, None, None).await.unwrap();
    assert_eq!(vendor_bills.len(), 3);
}

#[tokio::test]
async fn test_update_bill_status() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let bill_service = BillService::new_with_cache(cache_service.clone());
    let contact_service = ContactService::new_with_cache(cache_service.clone());
    let account_service = AccountService::new_with_cache(cache_service);

    // Setup
    let vendor_req = CreateContactRequest {
        contact_type: ContactType::Vendor,
        name: "Test Vendor 4".to_string(),
        email: Some("vendor4@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };
    let vendor = contact_service.create_contact(pool, vendor_req).await.unwrap();

    let expense_account_req = CreateAccountRequest {
        code: "5003".to_string(),
        name: "Professional Services".to_string(),
        account_type: AccountType::Expense,
        parent_account_id: None,
        company_id: None,
    };
    let expense_account = account_service.create_account(pool, expense_account_req).await.unwrap();

    // Create bill
    let bill_req = CreateBillRequest {
        bill_number: Some("BILL-STATUS-001".to_string()),
        vendor_id: vendor.id,
        bill_date: NaiveDate::from_ymd_opt(2024, 10, 10).unwrap(),
        due_date: NaiveDate::from_ymd_opt(2024, 11, 10).unwrap(),
        memo: None,
        company_id: None,
        line_items: vec![
            CreateBillLineItemRequest {
                line_number: 1,
                description: Some("Legal services".to_string()),
                amount: Decimal::from_str("1500.00").unwrap(),
                expense_account_id: expense_account.id,
                billable: Some(true),
                customer_id: None,
            }
        ],
    };
    let bill = bill_service.create_bill(pool, bill_req).await.unwrap();

    // Verify initial status
    assert_eq!(bill.status, BillStatus::Open);

    // Update status to void
    let updated_bill = bill_service.update_bill_status(pool, bill.id, BillStatus::Void).await.unwrap();
    assert_eq!(updated_bill.status, BillStatus::Void);
}

#[tokio::test]
async fn test_delete_bill() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let bill_service = BillService::new_with_cache(cache_service.clone());
    let contact_service = ContactService::new_with_cache(cache_service.clone());
    let account_service = AccountService::new_with_cache(cache_service);

    // Setup
    let vendor_req = CreateContactRequest {
        contact_type: ContactType::Vendor,
        name: "Test Vendor 5".to_string(),
        email: Some("vendor5@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };
    let vendor = contact_service.create_contact(pool, vendor_req).await.unwrap();

    let expense_account_req = CreateAccountRequest {
        code: "5004".to_string(),
        name: "Marketing Expense".to_string(),
        account_type: AccountType::Expense,
        parent_account_id: None,
        company_id: None,
    };
    let expense_account = account_service.create_account(pool, expense_account_req).await.unwrap();

    // Create bill
    let bill_req = CreateBillRequest {
        bill_number: Some("BILL-DELETE-001".to_string()),
        vendor_id: vendor.id,
        bill_date: NaiveDate::from_ymd_opt(2024, 10, 15).unwrap(),
        due_date: NaiveDate::from_ymd_opt(2024, 11, 15).unwrap(),
        memo: None,
        company_id: None,
        line_items: vec![
            CreateBillLineItemRequest {
                line_number: 1,
                description: Some("Marketing campaign".to_string()),
                amount: Decimal::from_str("2500.00").unwrap(),
                expense_account_id: expense_account.id,
                billable: Some(false),
                customer_id: None,
            }
        ],
    };
    let bill = bill_service.create_bill(pool, bill_req).await.unwrap();

    // Delete the bill
    bill_service.delete_bill(pool, bill.id).await.unwrap();

    // Verify bill was deleted
    let deleted_bill = bill_service.get_bill_by_id(pool, bill.id).await.unwrap();
    assert!(deleted_bill.is_none());
}

#[tokio::test]
async fn test_get_vendor_bills() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let bill_service = BillService::new_with_cache(cache_service.clone());
    let contact_service = ContactService::new_with_cache(cache_service.clone());
    let account_service = AccountService::new_with_cache(cache_service);

    // Create vendor
    let vendor_req = CreateContactRequest {
        contact_type: ContactType::Vendor,
        name: "Test Vendor 6".to_string(),
        email: Some("vendor6@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };
    let vendor = contact_service.create_contact(pool, vendor_req).await.unwrap();
// Create expense account
let expense_account_req = CreateAccountRequest {
    code: "5005".to_string(),
    name: "Travel Expense".to_string(),
    account_type: AccountType::Expense,
    parent_account_id: None,
    company_id: None,
};
let expense_account = account_service.create_account(pool, expense_account_req).await.unwrap();

// Create multiple bills for the vendor
for i in 1..=3 {
    let bill_req = CreateBillRequest {
        bill_number: Some(format!("VENDOR-BILL-{}", i)),
        vendor_id: vendor.id,
        bill_date: NaiveDate::from_ymd_opt(2024, 10, i as u32).unwrap(),
        due_date: NaiveDate::from_ymd_opt(2024, 11, i as u32).unwrap(),
        memo: None,
        company_id: None,
        line_items: vec![
            CreateBillLineItemRequest {
                line_number: 1,
                description: Some(format!("Travel expense {}", i)),
                amount: Decimal::from_str(&format!("{}.00", i * 200)).unwrap(),
                expense_account_id: expense_account.id,
                billable: Some(false),
                customer_id: None,
            }
        ],
    };
    bill_service.create_bill(pool, bill_req).await.unwrap();
}

// Get vendor bills
let vendor_bills = bill_service.get_vendor_bills(pool, vendor.id).await.unwrap();
assert_eq!(vendor_bills.len(), 3);

// Verify all bills belong to the vendor
assert!(vendor_bills.iter().all(|b| b.vendor_id == vendor.id));
}

#[tokio::test]
async fn test_get_overdue_bills() {
let test_db = TestDb::new().await;
let pool = test_db.pool();

let cache_service = CacheService::new("redis://localhost:6379").unwrap();
let bill_service = BillService::new_with_cache(cache_service.clone());
let contact_service = ContactService::new_with_cache(cache_service.clone());
let account_service = AccountService::new_with_cache(cache_service);

// Setup
let vendor_req = CreateContactRequest {
    contact_type: ContactType::Vendor,
    name: "Test Vendor 7".to_string(),
    email: Some("vendor7@test.com".to_string()),
    phone: None,
    billing_address: None,
    shipping_address: None,
    company_id: None,
};
let vendor = contact_service.create_contact(pool, vendor_req).await.unwrap();

let expense_account_req = CreateAccountRequest {
    code: "5006".to_string(),
    name: "Insurance Expense".to_string(),
    account_type: AccountType::Expense,
    parent_account_id: None,
    company_id: None,
};
let expense_account = account_service.create_account(pool, expense_account_req).await.unwrap();

// Create an overdue bill (due date in the past)
let bill_req = CreateBillRequest {
    bill_number: Some("BILL-OVERDUE-001".to_string()),
    vendor_id: vendor.id,
    bill_date: NaiveDate::from_ymd_opt(2024, 8, 1).unwrap(),
    due_date: NaiveDate::from_ymd_opt(2024, 9, 1).unwrap(), // Past due date
    memo: Some("Overdue insurance bill".to_string()),
    company_id: None,
    line_items: vec![
        CreateBillLineItemRequest {
            line_number: 1,
            description: Some("Annual insurance premium".to_string()),
            amount: Decimal::from_str("3000.00").unwrap(),
            expense_account_id: expense_account.id,
            billable: Some(false),
            customer_id: None,
        }
    ],
};
bill_service.create_bill(pool, bill_req).await.unwrap();

// Get overdue bills
let overdue_bills = bill_service.get_overdue_bills(pool).await.unwrap();

// Should have at least one overdue bill
assert!(!overdue_bills.is_empty());

// Verify all returned bills are overdue
let today = chrono::Utc::now().date_naive();
assert!(overdue_bills.iter().all(|b| b.due_date < today));
}

#[tokio::test]
async fn test_bill_total_calculation() {
let test_db = TestDb::new().await;
let pool = test_db.pool();

let cache_service = CacheService::new("redis://localhost:6379").unwrap();
let bill_service = BillService::new_with_cache(cache_service.clone());
let contact_service = ContactService::new_with_cache(cache_service.clone());
let account_service = AccountService::new_with_cache(cache_service);

// Setup
let vendor_req = CreateContactRequest {
    contact_type: ContactType::Vendor,
    name: "Test Vendor 8".to_string(),
    email: Some("vendor8@test.com".to_string()),
    phone: None,
    billing_address: None,
    shipping_address: None,
    company_id: None,
};
let vendor = contact_service.create_contact(pool, vendor_req).await.unwrap();

let expense_account_req = CreateAccountRequest {
    code: "5007".to_string(),
    name: "Equipment Expense".to_string(),
    account_type: AccountType::Expense,
    parent_account_id: None,
    company_id: None,
};
let expense_account = account_service.create_account(pool, expense_account_req).await.unwrap();

// Create bill with multiple line items
let bill_req = CreateBillRequest {
    bill_number: Some("BILL-CALC-001".to_string()),
    vendor_id: vendor.id,
    bill_date: NaiveDate::from_ymd_opt(2024, 10, 20).unwrap(),
    due_date: NaiveDate::from_ymd_opt(2024, 11, 20).unwrap(),
    memo: None,
    company_id: None,
    line_items: vec![
        CreateBillLineItemRequest {
            line_number: 1,
            description: Some("Computer".to_string()),
            amount: Decimal::from_str("1200.00").unwrap(),
            expense_account_id: expense_account.id,
            billable: Some(false),
            customer_id: None,
        },
        CreateBillLineItemRequest {
            line_number: 2,
            description: Some("Monitor".to_string()),
            amount: Decimal::from_str("350.00").unwrap(),
            expense_account_id: expense_account.id,
            billable: Some(false),
            customer_id: None,
        },
        CreateBillLineItemRequest {
            line_number: 3,
            description: Some("Keyboard and Mouse".to_string()),
            amount: Decimal::from_str("125.50").unwrap(),
            expense_account_id: expense_account.id,
            billable: Some(false),
            customer_id: None,
        }
    ],
};

let bill = bill_service.create_bill(pool, bill_req).await.unwrap();

// Verify total is correctly calculated
let expected_total = Decimal::from_str("1675.50").unwrap(); // 1200 + 350 + 125.50
assert_eq!(bill.total_amount, expected_total);
assert_eq!(bill.balance, expected_total);
}
        