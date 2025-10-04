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
};
use crate::utils::{ApiResponse, HealthResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::health_check,
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
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "accounts", description = "Chart of Accounts management"),
        (name = "transactions", description = "Transaction management with double-entry bookkeeping"),
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
            - Account balance calculation\n\n\
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
