use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

use crate::models::{
    // User models
    User, UserRole, UserResponse, CreateUserRequest, LoginRequest, AuthResponse,
    // Account models
    Account, AccountType, CreateAccountRequest, UpdateAccountRequest,
    // Transaction models
    Transaction, TransactionLineItem, TransactionStatus, JournalType,
    CreateTransactionRequest, CreateLineItemRequest, TransactionWithLineItems,
    // Contact models
    Contact, ContactType, CreateContactRequest, UpdateContactRequest,
    // Invoice models
    Invoice, InvoiceLineItem, InvoiceStatus, CreateInvoiceRequest,
    CreateInvoiceLineItemRequest, InvoiceWithLineItems,
    // Payment models
    Payment, PaymentApplication, CreatePaymentRequest, PaymentApplicationRequest,
    BillPayment, CreateBillPaymentRequest, BillPaymentApplicationRequest,
    // Bill models
    Bill, BillLineItem, BillStatus, CreateBillRequest, CreateBillLineItemRequest,
    BillWithLineItems,
    // Reporting models
    TrialBalance, TrialBalanceEntry, ProfitLossStatement, ProfitLossEntry,
    BalanceSheet, BalanceSheetEntry, AccountsReceivableAging, AgingBucket,
    DateRangeRequest, DateRequest,
};
use crate::utils::{ApiResponse, HealthResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Health
        crate::routes::health_check,
        // Auth
        crate::handlers::auth::register,
        crate::handlers::auth::login,
        crate::handlers::auth::refresh_token,
        crate::handlers::auth::me,
        // Accounts
        crate::handlers::account::list_accounts,
        crate::handlers::account::create_account,
        crate::handlers::account::get_account,
        crate::handlers::account::update_account,
        crate::handlers::account::deactivate_account,
        crate::handlers::account::get_account_hierarchy,
        // Transactions
        crate::handlers::transaction::list_transactions,
        crate::handlers::transaction::create_transaction,
        crate::handlers::transaction::get_transaction,
        crate::handlers::transaction::update_transaction_status,
        crate::handlers::transaction::delete_transaction,
        crate::handlers::transaction::get_account_balance,
        // Contacts
        crate::handlers::contact::list_contacts,
        crate::handlers::contact::create_contact,
        crate::handlers::contact::get_contact,
        crate::handlers::contact::update_contact,
        crate::handlers::contact::delete_contact,
        crate::handlers::contact::get_customers,
        crate::handlers::contact::get_vendors,
        crate::handlers::contact::get_employees,
        // Invoices
        crate::handlers::invoice::create_invoice,
        crate::handlers::invoice::list_invoices,
        crate::handlers::invoice::get_invoice,
        crate::handlers::invoice::update_invoice_status,
        crate::handlers::invoice::get_customer_invoices,
        crate::handlers::invoice::get_overdue_invoices,
        crate::handlers::payment::get_invoice_payments,
        // Payments
        crate::handlers::payment::create_payment,
        crate::handlers::payment::list_payments,
        crate::handlers::payment::get_payment,
        crate::handlers::payment::apply_payment,
        crate::handlers::payment::get_unapplied_payments,
        crate::handlers::payment::create_bill_payment,
        // Bills
        crate::handlers::bill::create_bill,
        crate::handlers::bill::list_bills,
        crate::handlers::bill::get_bill,
        crate::handlers::bill::update_bill_status,
        crate::handlers::bill::delete_bill,
        crate::handlers::bill::get_vendor_bills,
        crate::handlers::bill::get_overdue_bills,
        // Reporting
        crate::handlers::reporting::get_trial_balance,
        crate::handlers::reporting::get_profit_loss,
        crate::handlers::reporting::get_balance_sheet,
        crate::handlers::reporting::get_ar_aging,
    ),
    components(
        schemas(
            // Response types
            ApiResponse<User>,
            ApiResponse<UserResponse>,
            ApiResponse<AuthResponse>,
            ApiResponse<Vec<Account>>,
            ApiResponse<Account>,
            ApiResponse<Vec<Transaction>>,
            ApiResponse<Transaction>,
            ApiResponse<TransactionWithLineItems>,
            ApiResponse<Vec<Contact>>,
            ApiResponse<Contact>,
            ApiResponse<Vec<Invoice>>,
            ApiResponse<Invoice>,
            ApiResponse<InvoiceWithLineItems>,
            ApiResponse<Vec<Payment>>,
            ApiResponse<Payment>,
            ApiResponse<BillPayment>,
            ApiResponse<String>,
            ApiResponse<TrialBalance>,
            ApiResponse<ProfitLossStatement>,
            ApiResponse<BalanceSheet>,
            ApiResponse<AccountsReceivableAging>,
            HealthResponse,
            // User types
            User,
            UserRole,
            UserResponse,
            CreateUserRequest,
            LoginRequest,
            AuthResponse,
            // Account types
            Account,
            AccountType,
            CreateAccountRequest,
            UpdateAccountRequest,
            // Transaction types
            Transaction,
            TransactionLineItem,
            TransactionStatus,
            JournalType,
            CreateTransactionRequest,
            CreateLineItemRequest,
            TransactionWithLineItems,
            // Contact types
            Contact,
            ContactType,
            CreateContactRequest,
            UpdateContactRequest,
            // Invoice types
            Invoice,
            InvoiceLineItem,
            InvoiceStatus,
            CreateInvoiceRequest,
            CreateInvoiceLineItemRequest,
            InvoiceWithLineItems,
            // Payment types
            Payment,
            PaymentApplication,
            CreatePaymentRequest,
            PaymentApplicationRequest,
            BillPayment,
            CreateBillPaymentRequest,
            BillPaymentApplicationRequest,
            // Bill types
            Bill,
            BillLineItem,
            BillStatus,
            CreateBillRequest,
            CreateBillLineItemRequest,
            BillWithLineItems,
            // Reporting types
            TrialBalance,
            TrialBalanceEntry,
            ProfitLossStatement,
            ProfitLossEntry,
            BalanceSheet,
            BalanceSheetEntry,
            AccountsReceivableAging,
            AgingBucket,
            DateRangeRequest,
            DateRequest,
            // Additional request/response types
            crate::handlers::auth::RefreshTokenRequest,
            crate::handlers::auth::TokenResponse,
            crate::handlers::transaction::UpdateStatusRequest,
            crate::handlers::invoice::UpdateInvoiceStatusRequest,
            crate::handlers::payment::ApplyPaymentRequest,
            crate::handlers::bill::UpdateBillStatusRequest,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "accounts", description = "Chart of Accounts management"),
        (name = "transactions", description = "Transaction management with double-entry bookkeeping"),
        (name = "contacts", description = "Contact management (Customers, Vendors, Employees)"),
        (name = "invoices", description = "Invoice management with line items and status tracking"),
        (name = "payments", description = "Payment processing and application to invoices"),
        (name = "bills", description = "Bill management (Accounts Payable)"),
        (name = "reporting", description = "Financial reporting and analysis"),
    ),
    info(
        title = "LedgerForge API",
        version = "0.1.0",
        description = "A high-performance, Rust-powered accounting system with double-entry bookkeeping.\n\n\
            ## Features\n\
            - JWT authentication with secure token management\n\
            - Chart of Accounts with hierarchical structure\n\
            - Double-entry transaction engine\n\
            - Transaction status workflow (draft → posted → void)\n\
            - Account balance calculation\n\
            - Invoice management with line items and payment tracking\n\
            - Payment processing and application\n\
            - Financial reporting (Trial Balance, P&L, Balance Sheet, A/R Aging)\n\n\
            ## Authentication\n\
            Most endpoints require JWT authentication. Include the token in the Authorization header:\n\
            ```\n\
            Authorization: Bearer <your-access-token>\n\
            ```",
        contact(
            name = "LedgerForge Support",
            email = "support@ledgerforge.example.com"
        ),
        license(
            name = "Proprietary",
        ),
    ),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .description(Some("Enter your JWT token"))
                        .build()
                ),
            )
        }
    }
}
