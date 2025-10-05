use ledger_forge::models::{Contact, ContactType, CreateContactRequest, UpdateContactRequest};
use ledger_forge::services::ContactService;
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

// Helper function to get database URL from environment
fn get_database_url() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests")
}

// Helper function to create test database pool
async fn create_test_pool() -> sqlx::PgPool {
    let database_url = get_database_url();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

// Helper function to clean up test data
async fn cleanup_contacts(pool: &sqlx::PgPool) {
    sqlx::query("DELETE FROM contacts WHERE name LIKE 'Test%' OR email LIKE 'test%'")
        .execute(pool)
        .await
        .expect("Failed to clean up test contacts");
}

#[tokio::test]
#[serial]
async fn test_create_contact_customer() {
    let pool = create_test_pool().await;
    cleanup_contacts(&pool).await;

    let service = ContactService::new();

    let request = CreateContactRequest {
        contact_type: ContactType::Customer,
        name: "Test Customer Corp".to_string(),
        email: Some("customer@test.com".to_string()),
        phone: Some("+1-555-0100".to_string()),
        billing_address: Some("123 Main St".to_string()),
        shipping_address: Some("456 Oak Ave".to_string()),
        company_id: None,
    };

    let contact = service
        .create_contact(&pool, request)
        .await
        .expect("Failed to create contact");

    assert_eq!(contact.name, "Test Customer Corp");
    assert_eq!(contact.contact_type, ContactType::Customer);
    assert_eq!(contact.email, Some("customer@test.com".to_string()));
    assert_eq!(contact.phone, Some("+1-555-0100".to_string()));

    cleanup_contacts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_create_contact_vendor() {
    let pool = create_test_pool().await;
    cleanup_contacts(&pool).await;

    let service = ContactService::new();

    let request = CreateContactRequest {
        contact_type: ContactType::Vendor,
        name: "Test Vendor LLC".to_string(),
        email: Some("vendor@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };

    let contact = service
        .create_contact(&pool, request)
        .await
        .expect("Failed to create vendor");

    assert_eq!(contact.name, "Test Vendor LLC");
    assert_eq!(contact.contact_type, ContactType::Vendor);
    assert_eq!(contact.email, Some("vendor@test.com".to_string()));
    assert!(contact.phone.is_none());

    cleanup_contacts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_create_contact_employee() {
    let pool = create_test_pool().await;
    cleanup_contacts(&pool).await;

    let service = ContactService::new();

    let request = CreateContactRequest {
        contact_type: ContactType::Employee,
        name: "Test Employee".to_string(),
        email: Some("employee@test.com".to_string()),
        phone: Some("+1-555-0102".to_string()),
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };

    let contact = service
        .create_contact(&pool, request)
        .await
        .expect("Failed to create employee");

    assert_eq!(contact.name, "Test Employee");
    assert_eq!(contact.contact_type, ContactType::Employee);

    cleanup_contacts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_get_contact_by_id() {
    let pool = create_test_pool().await;
    cleanup_contacts(&pool).await;

    let service = ContactService::new();

    // Create a contact
    let request = CreateContactRequest {
        contact_type: ContactType::Customer,
        name: "Test GetById Customer".to_string(),
        email: Some("getbyid@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };

    let created = service
        .create_contact(&pool, request)
        .await
        .expect("Failed to create contact");

    // Retrieve it
    let retrieved = service
        .get_contact_by_id(&pool, created.id)
        .await
        .expect("Failed to get contact");

    assert_eq!(retrieved.id, created.id);
    assert_eq!(retrieved.name, "Test GetById Customer");
    assert_eq!(retrieved.email, Some("getbyid@test.com".to_string()));

    cleanup_contacts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_get_contact_by_id_not_found() {
    let pool = create_test_pool().await;
    let service = ContactService::new();

    let non_existent_id = Uuid::new_v4();
    let result = service.get_contact_by_id(&pool, non_existent_id).await;

    assert!(result.is_err());
}

#[tokio::test]
#[serial]
async fn test_list_contacts() {
    let pool = create_test_pool().await;
    cleanup_contacts(&pool).await;

    let service = ContactService::new();

    // Create multiple contacts
    let customer_req = CreateContactRequest {
        contact_type: ContactType::Customer,
        name: "Test List Customer 1".to_string(),
        email: Some("listcust1@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };

    let vendor_req = CreateContactRequest {
        contact_type: ContactType::Vendor,
        name: "Test List Vendor 1".to_string(),
        email: Some("listvend1@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };

    service.create_contact(&pool, customer_req).await.unwrap();
    service.create_contact(&pool, vendor_req).await.unwrap();

    // List all contacts
    let all_contacts = service
        .list_contacts(&pool, None, None, None)
        .await
        .expect("Failed to list contacts");

    assert!(all_contacts.len() >= 2);

    cleanup_contacts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_list_contacts_filtered_by_type() {
    let pool = create_test_pool().await;
    cleanup_contacts(&pool).await;

    let service = ContactService::new();

    // Create contacts of different types
    let customer_req = CreateContactRequest {
        contact_type: ContactType::Customer,
        name: "Test Filter Customer".to_string(),
        email: Some("filtercust@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };

    let vendor_req = CreateContactRequest {
        contact_type: ContactType::Vendor,
        name: "Test Filter Vendor".to_string(),
        email: Some("filtervend@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };

    service.create_contact(&pool, customer_req).await.unwrap();
    service.create_contact(&pool, vendor_req).await.unwrap();

    // Filter by customer type
    let customers = service
        .list_contacts(&pool, Some(ContactType::Customer), None, None)
        .await
        .expect("Failed to list customers");

    assert!(customers.iter().all(|c| c.contact_type == ContactType::Customer));

    // Filter by vendor type
    let vendors = service
        .list_contacts(&pool, Some(ContactType::Vendor), None, None)
        .await
        .expect("Failed to list vendors");

    assert!(vendors.iter().all(|c| c.contact_type == ContactType::Vendor));

    cleanup_contacts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_list_contacts_with_limit() {
    let pool = create_test_pool().await;
    cleanup_contacts(&pool).await;

    let service = ContactService::new();

    // Create 5 contacts
    for i in 1..=5 {
        let req = CreateContactRequest {
            contact_type: ContactType::Customer,
            name: format!("Test Limit Customer {}", i),
            email: Some(format!("limitcust{}@test.com", i)),
            phone: None,
            billing_address: None,
            shipping_address: None,
            company_id: None,
        };
        service.create_contact(&pool, req).await.unwrap();
    }

    // List with limit of 3
    let limited = service
        .list_contacts(&pool, None, None, Some(3))
        .await
        .expect("Failed to list with limit");

    assert_eq!(limited.len(), 3);

    cleanup_contacts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_update_contact() {
    let pool = create_test_pool().await;
    cleanup_contacts(&pool).await;

    let service = ContactService::new();

    // Create a contact
    let request = CreateContactRequest {
        contact_type: ContactType::Customer,
        name: "Test Update Customer".to_string(),
        email: Some("update@test.com".to_string()),
        phone: Some("+1-555-0103".to_string()),
        billing_address: Some("Old Address".to_string()),
        shipping_address: None,
        company_id: None,
    };

    let created = service
        .create_contact(&pool, request)
        .await
        .expect("Failed to create contact");

    // Update the contact
    let update_req = UpdateContactRequest {
        name: Some("Test Updated Customer".to_string()),
        email: Some("updated@test.com".to_string()),
        phone: None,
        billing_address: Some("New Address".to_string()),
        shipping_address: Some("New Shipping".to_string()),
    };

    let updated = service
        .update_contact(&pool, created.id, update_req)
        .await
        .expect("Failed to update contact");

    assert_eq!(updated.name, "Test Updated Customer");
    assert_eq!(updated.email, Some("updated@test.com".to_string()));
    assert_eq!(updated.billing_address, Some("New Address".to_string()));
    assert_eq!(updated.shipping_address, Some("New Shipping".to_string()));
    // Phone should be cleared (set to None in update)
    assert!(updated.phone.is_none());

    cleanup_contacts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_update_contact_partial() {
    let pool = create_test_pool().await;
    cleanup_contacts(&pool).await;

    let service = ContactService::new();

    // Create a contact
    let request = CreateContactRequest {
        contact_type: ContactType::Vendor,
        name: "Test Partial Update Vendor".to_string(),
        email: Some("partial@test.com".to_string()),
        phone: Some("+1-555-0104".to_string()),
        billing_address: Some("Original Address".to_string()),
        shipping_address: None,
        company_id: None,
    };

    let created = service
        .create_contact(&pool, request)
        .await
        .expect("Failed to create contact");

    // Update only the name
    let update_req = UpdateContactRequest {
        name: Some("Test Partially Updated Vendor".to_string()),
        email: None,
        phone: None,
        billing_address: None,
        shipping_address: None,
    };

    let updated = service
        .update_contact(&pool, created.id, update_req)
        .await
        .expect("Failed to update contact");

    assert_eq!(updated.name, "Test Partially Updated Vendor");
    // Other fields should remain unchanged
    assert_eq!(updated.email, Some("partial@test.com".to_string()));
    assert_eq!(updated.phone, Some("+1-555-0104".to_string()));
    assert_eq!(updated.billing_address, Some("Original Address".to_string()));

    cleanup_contacts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_delete_contact() {
    let pool = create_test_pool().await;
    cleanup_contacts(&pool).await;

    let service = ContactService::new();

    // Create a contact
    let request = CreateContactRequest {
        contact_type: ContactType::Customer,
        name: "Test Delete Customer".to_string(),
        email: Some("delete@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };

    let created = service
        .create_contact(&pool, request)
        .await
        .expect("Failed to create contact");

    // Delete the contact
    service
        .delete_contact(&pool, created.id)
        .await
        .expect("Failed to delete contact");

    // Verify it's deleted
    let result = service.get_contact_by_id(&pool, created.id).await;
    assert!(result.is_err());

    cleanup_contacts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_get_customers() {
    let pool = create_test_pool().await;
    cleanup_contacts(&pool).await;

    let service = ContactService::new();

    // Create customers and vendors
    let customer_req = CreateContactRequest {
        contact_type: ContactType::Customer,
        name: "Test GetCustomers Customer".to_string(),
        email: Some("getcust@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };

    let vendor_req = CreateContactRequest {
        contact_type: ContactType::Vendor,
        name: "Test GetCustomers Vendor".to_string(),
        email: Some("getvend@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };

    service.create_contact(&pool, customer_req).await.unwrap();
    service.create_contact(&pool, vendor_req).await.unwrap();

    // Get only customers
    let customers = service
        .get_customers(&pool)
        .await
        .expect("Failed to get customers");

    assert!(customers.iter().all(|c| c.contact_type == ContactType::Customer));

    cleanup_contacts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_get_vendors() {
    let pool = create_test_pool().await;
    cleanup_contacts(&pool).await;

    let service = ContactService::new();

    // Create customers and vendors
    let customer_req = CreateContactRequest {
        contact_type: ContactType::Customer,
        name: "Test GetVendors Customer".to_string(),
        email: Some("getvendcust@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };

    let vendor_req = CreateContactRequest {
        contact_type: ContactType::Vendor,
        name: "Test GetVendors Vendor".to_string(),
        email: Some("getvendvend@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };

    service.create_contact(&pool, customer_req).await.unwrap();
    service.create_contact(&pool, vendor_req).await.unwrap();

    // Get only vendors
    let vendors = service
        .get_vendors(&pool)
        .await
        .expect("Failed to get vendors");

    assert!(vendors.iter().all(|c| c.contact_type == ContactType::Vendor));

    cleanup_contacts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_get_employees() {
    let pool = create_test_pool().await;
    cleanup_contacts(&pool).await;

    let service = ContactService::new();

    // Create different types of contacts
    let employee_req = CreateContactRequest {
        contact_type: ContactType::Employee,
        name: "Test GetEmployees Employee".to_string(),
        email: Some("getemp@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };

    let customer_req = CreateContactRequest {
        contact_type: ContactType::Customer,
        name: "Test GetEmployees Customer".to_string(),
        email: Some("getempcust@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };

    service.create_contact(&pool, employee_req).await.unwrap();
    service.create_contact(&pool, customer_req).await.unwrap();

    // Get only employees
    let employees = service
        .get_employees(&pool)
        .await
        .expect("Failed to get employees");

    assert!(employees.iter().all(|c| c.contact_type == ContactType::Employee));

    cleanup_contacts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_contact_validation_empty_name() {
    let pool = create_test_pool().await;
    let service = ContactService::new();

    let request = CreateContactRequest {
        contact_type: ContactType::Customer,
        name: "".to_string(), // Empty name should fail validation
        email: Some("valid@test.com".to_string()),
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };

    let result = service.create_contact(&pool, request).await;
    assert!(result.is_err());
}

#[tokio::test]
#[serial]
async fn test_contact_validation_invalid_email() {
    let pool = create_test_pool().await;
    let service = ContactService::new();

    let request = CreateContactRequest {
        contact_type: ContactType::Vendor,
        name: "Test Invalid Email Vendor".to_string(),
        email: Some("invalid-email".to_string()), // Invalid email format
        phone: None,
        billing_address: None,
        shipping_address: None,
        company_id: None,
    };

    let result = service.create_contact(&pool, request).await;
    assert!(result.is_err());
}
