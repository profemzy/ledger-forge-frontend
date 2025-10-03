









# Project LedgerForge: A Rust-Powered Accounting System Design Document

> **📋 Implementation Status (October 2025):**
> This is the original design blueprint. For actual implementation details, enhancements, and technology choices, see:
> - **[Design Implementation Notes](docs/DESIGN_IMPLEMENTATION_NOTES.md)** - Actual vs planned differences
> - **[Phase 1 Complete](docs/PHASE1_COMPLETE.md)** - Database foundation completion summary
> - **[Progress Tracker](docs/PROGRESS.md)** - Current development status

## 1. Introduction

### 1.1. Project Vision
The vision for "Project LedgerForge" is to engineer a bespoke, high-performance, and exceptionally secure accounting system tailored specifically to the unique operational and strategic requirements of the business. This system will serve as a complete replacement for existing off-the-shelf solutions like QuickBooks, offering not just functional parity but a superior level of customization, control, and insight. By leveraging the Rust programming language and its modern ecosystem, we aim to build a financial management platform that is robust, reliable, and capable of evolving with the business for years to come. The core of this vision is to move beyond the constraints of generic software and create a tool that feels like a natural extension of the business itself, empowering users with intuitive workflows, powerful reporting, and the confidence that their financial data is managed with the utmost integrity and security. This endeavor is an investment in financial autonomy, providing a competitive advantage through a deeply integrated and intelligent financial core.

### 1.2. Scope and Objectives
The primary objective of Project LedgerForge is to develop a comprehensive accounting application that replicates and enhances the core functionalities of QuickBooks, while ensuring a seamless migration of existing historical data. The scope of this project includes, but is not limited to:
*   **User Authentication and Authorization:** Implementing a secure system for managing user access with role-based permissions.
*   **Chart of Accounts Management:** Providing a flexible interface for creating and managing a custom chart of accounts.
*   **Double-Entry Accounting Engine:** Building a robust core that enforces the principles of double-entry bookkeeping for all financial transactions.
*   **Invoicing and Payment Processing:** Enabling the creation of professional invoices, tracking customer payments, and managing accounts receivable.
*   **Expense Tracking and Management:** Allowing for the recording, categorization, and tracking of business expenses and accounts payable.
*   **Financial Reporting:** Generating standard financial statements such as Profit and Loss, Balance Sheet, and Cash Flow statements, with options for customization.
*   **Bank Reconciliation:** Providing tools to reconcile bank accounts with the system's records.
*   **Data Import from QuickBooks:** Developing a reliable process to export data from the existing QuickBooks instance and import it into the new system, preserving data integrity and historical context [[30](https://quickbooks.intuit.com/learn-support/en-us/help-article/manage-lists/import-export-csv-files/L9AiGRdT9_US_en_US)], [[31](https://quickbooks.intuit.com/learn-support/en-us/help-article/import-export-data-files/import-export-data-quickbooks-desktop/L9KS42UxP_US_en_US)].
*   **System Scalability and Performance:** Designing the system to handle growing volumes of data and users without compromising performance.
*   **Security:** Ensuring the highest levels of data security and protection against unauthorized access.

### 1.3. Target Audience
The primary users of Project LedgerForge will be the internal stakeholders of the business responsible for financial management. This includes:
*   **Business Owners/Executives:** Who require high-level financial reports and dashboards for strategic decision-making.
*   **Accounting Staff:** Who will be the primary users for day-to-day transaction entry, invoice management, expense tracking, and bank reconciliation.
*   **IT Personnel:** Who will be responsible for system administration, maintenance, and potentially, further development.

The system will be designed with a focus on usability for accounting professionals, while also providing the advanced features and security required by management and IT.

### 1.4. Definitions and Acronyms
*   **Rust:** A systems programming language focused on safety, speed, and concurrency.
*   **Actix Web / Axum:** High-performance web frameworks for Rust.
*   **Leptos / Dioxus / Yew:** Modern frontend frameworks for Rust, capable of compiling to WebAssembly.
*   **SQLx:** An async, pure Rust SQL toolkit.
*   **PostgreSQL:** A powerful, open-source object-relational database system.
*   **API (Application Programming Interface):** A set of protocols and tools for building software applications.
*   **SSR (Server-Side Rendering):** A technique where web pages are generated on the server.
*   **CSR (Client-Side Rendering):** A technique where web pages are rendered in the browser using JavaScript (or WebAssembly).
*   **ORM (Object-Relational Mapping):** A technique for converting data between incompatible type systems using object-oriented programming languages.
*   **CRUD (Create, Read, Update, Delete):** The four basic operations of persistent storage.
*   **GAAP (Generally Accepted Accounting Principles):** A common set of accounting rules, standards, and procedures.
*   **CSV (Comma-Separated Values):** A simple file format used to store tabular data.

## 2. System Architecture

### 2.1. Architectural Overview
Project LedgerForge will adopt a modular, service-oriented architecture, even if initially deployed as a monolith, to facilitate future scalability and maintainability. The system will be broadly divided into three main layers:

1.  **Frontend (Client-Side):** This layer will be responsible for the user interface and user experience. It will be built using a modern Rust frontend framework that compiles to WebAssembly, offering high performance and type safety across the full stack. The frontend will communicate with the backend via a RESTful API or GraphQL, fetching data and sending user interactions.
2.  **Backend (Server-Side):** This layer will handle the application logic, data processing, user authentication, authorization, and API endpoints. Built with Rust and a robust web framework, it will be designed for concurrency, safety, and performance. The backend will interact with the database to persist and retrieve data.
3.  **Database (Data Layer):** This layer will be responsible for the persistent storage of all financial data. PostgreSQL is the recommended choice due to its reliability, advanced features, and strong compliance with SQL standards. The database schema will be meticulously designed to ensure data integrity, support complex queries, and facilitate efficient reporting.

This separation of concerns allows for independent development, testing, and scaling of each component. The use of Rust for both frontend and backend (via WebAssembly) offers the potential for significant code reuse and a unified development experience.

### 2.2. Technology Stack

The choice of technology stack is driven by the requirements for performance, safety, concurrency, and developer productivity, aligning with the user's preference for Rust.

| Component | Technology | Rationale |
| :--- | :--- | :--- |
| **Backend Language** | **Rust** | Provides memory safety without a garbage collector, high performance, strong concurrency support, and a growing ecosystem. Ideal for building reliable and fast backend services [[9](https://www.shuttle.dev/blog/2024/07/31/rust-on-the-backend)]. |
| **Backend Framework** | **Actix Web** or **Axum** | **Actix Web** is a mature, feature-rich, and extremely performant framework [[0](https://www.rustfinity.com/blog/best-rust-web-frameworks)], [[4](https://blog.logrocket.com/top-rust-web-frameworks)], [[6](https://randiekas.medium.com/rust-the-fastest-rust-web-framework-in-2024-cf738c40343b)]. **Axum** is a modern, ergonomic library from the Tokio project, offering excellent integration with the async ecosystem [[0](https://www.rustfinity.com/blog/best-rust-web-frameworks)], [[5](https://users.rust-lang.org/t/rust-backend-framework/119845)]. Both are excellent choices; Actix Web for its proven track record in high-load scenarios, and Axum for its modern design and Tokio integration. |
| **Frontend Framework** | **Leptos** or **Dioxus** | These are leading Rust frontend frameworks that compile to WebAssembly, enabling high-performance client-side applications with type safety. **Leptos** offers fine-grained reactivity and supports both client-side and server-side rendering [[20](https://www.reddit.com/r/rust/comments/18schae/best_rust_web_ui_framework_for_2024)], [[22](https://blog.logrocket.com/top-rust-web-frameworks)], [[26](https://levelup.gitconnected.com/top-rust-frameworks-for-2024-part-2-b589280f207d)]. **Dioxus** is also highly capable and aims to be a cross-platform UI framework [[20](https://www.reddit.com/r/rust/comments/18schae/best_rust_web_ui_framework_for_2024)], [[21](https://news.ycombinator.com/item?id=39852831)]. The choice may depend on specific ergonomics and rendering strategy preferences (SSR vs CSR). |
| **Database** | **PostgreSQL** | A powerful, open-source, object-relational database system known for its reliability, robustness, feature set, and standards compliance. Well-suited for complex financial applications requiring data integrity. |
| **Database Toolkit** | **SQLx** | An async, pure Rust SQL toolkit that provides compile-time checked queries (without a full ORM), connection pooling, and support for multiple database backends including PostgreSQL. It offers a good balance between the safety of an ORM and the flexibility of raw SQL [[18](https://medium.com/@vishwajitpatil1224/3-rust-database-libraries-compared-sqlx-vs-diesel-vs-seaorm-4b978f96e1af)]. Diesel is another strong contender, particularly for its compile-time guarantees [[13](https://diesel.rs/compare_diesel.html)], [[15](https://rust-trends.com/posts/database-crates-diesel-sqlx-tokio-postgress)], but SQLx's async-first nature and flexibility might be preferable for this project. |
| **WebAssembly (Wasm)** | **(via Frontend Framework)** | Enables running Rust code in the browser, providing near-native performance for frontend logic and computations. |

### 2.3. Data Flow
A typical data flow within Project LedgerForge would involve:

1.  **User Interaction:** A user interacts with the frontend application (built with Leptos/Dioxus) in their web browser.
2.  **API Request:** The frontend application makes an HTTP request (e.g., GET, POST, PUT, DELETE) to an API endpoint exposed by the Rust backend (Actix Web/Axum).
3.  **Backend Processing:** The backend receives the request, performs authentication and authorization checks, processes the business logic (e.g., validates transaction data, applies accounting rules), and interacts with the database.
4.  **Database Interaction:** The backend uses SQLx to execute SQL queries against the PostgreSQL database to retrieve or modify data.
5.  **Response:** The backend formulates an HTTP response, typically containing JSON data, and sends it back to the frontend.
6.  **UI Update:** The frontend application receives the response, updates its state, and re-renders the UI to reflect the changes.

For complex reporting or data-intensive operations, the backend might perform significant data aggregation and processing before sending a summarized result to the frontend, optimizing client-side performance.

### 2.4. Deployment Architecture
The deployment architecture will aim for simplicity, reliability, and scalability.

*   **Application Server:** The Rust backend application will be compiled into a single binary. This binary can be deployed directly on a Linux server or containerized using Docker for easier management and portability.
*   **Web Server:** A reverse proxy like Nginx or Apache will sit in front of the Rust application. It will handle SSL/TLS termination, serve static assets if any (though the Rust frontend Wasm might be served by the app server or a CDN), and forward requests to the application server.
*   **Database Server:** PostgreSQL will run on a separate server or a managed database service for better performance, security, and backup/recovery capabilities.
*   **Hosting Environment:** Cloud platforms like AWS, Google Cloud Platform, or Azure offer robust infrastructure for hosting such an application. Services like AWS EC2 for the application server, RDS for PostgreSQL, and S3 for storing exported reports or attachments could be utilized. Alternatively, Platform as a Service (PaaS) offerings that support Rust could simplify deployment.
*   **CI/CD (Continuous Integration/Continuous Deployment):** A CI/CD pipeline (e.g., using GitHub Actions, GitLab CI) will be established to automate testing, building, and deployment processes, ensuring rapid and reliable releases.

## 3. Database Schema Design

A robust and well-normalized database schema is the cornerstone of any reliable accounting system. The following schema outlines the core tables for Project LedgerForge, adhering to double-entry accounting principles.

### 3.1. Core Entities

*   **`users`**
    *   `id` (UUID, Primary Key)
    *   `username` (VARCHAR, Unique, Not Null)
    *   `email` (VARCHAR, Unique, Not Null)
    *   `password_hash` (VARCHAR, Not Null) -- Store hashed passwords only.
    *   `role` (VARCHAR or ENUM, e.g., 'admin', 'accountant', 'viewer')
    *   `created_at` (TIMESTAMPTZ, Not Null, Default NOW())
    *   `updated_at` (TIMESTAMPTZ, Not Null, Default NOW())

*   **`companies`** (If multi-tenancy is a future possibility, or for storing company-specific settings)
    *   `id` (UUID, Primary Key)
    *   `name` (VARCHAR, Not Null)
    *   `address` (TEXT)
    *   `created_at` (TIMESTAMPTZ, Not Null, Default NOW())
    *   `updated_at` (TIMESTAMPTZ, Not Null, Default NOW())

*   **`chart_of_accounts`**
    *   `id` (UUID, Primary Key)
    *   `code` (VARCHAR, Not Null) -- e.g., "1000", "2010"
    *   `name` (VARCHAR, Not Null) -- e.g., "Checking Account", "Accounts Payable"
    *   `account_type` (VARCHAR or ENUM, Not Null) -- 'Asset', 'Liability', 'Equity', 'Revenue', 'Expense'
    *   `parent_account_id` (UUID, Nullable, Foreign Key to `chart_of_accounts.id`) -- For hierarchical accounts.
    *   `is_active` (BOOLEAN, Not Null, Default true)
    *   `company_id` (UUID, Nullable, Foreign Key to `companies.id`) -- If multi-tenant.
    *   `created_at` (TIMESTAMPTZ, Not Null, Default NOW())
    *   `updated_at` (TIMESTAMPTZ, Not Null, Default NOW())
    *   Unique constraint on (`code`, `company_id`) if multi-tenant, or just `code` otherwise.

*   **`contacts`** (Customers, Vendors, Employees)
    *   `id` (UUID, Primary Key)
    *   `contact_type` (VARCHAR or ENUM, Not Null) -- 'Customer', 'Vendor', 'Employee'
    *   `name` (VARCHAR, Not Null)
    *   `email` (VARCHAR)
    *   `phone` (VARCHAR)
    *   `billing_address` (TEXT)
    *   `shipping_address` (TEXT)
    *   `company_id` (UUID, Nullable, Foreign Key to `companies.id`)
    *   `created_at` (TIMESTAMPTZ, Not Null, Default NOW())
    *   `updated_at` (TIMESTAMPTZ, Not Null, Default NOW())

### 3.2. Transaction Engine

*   **`transactions`**
    *   `id` (UUID, Primary Key)
    *   `transaction_date` (DATE, Not Null)
    *   `description` (TEXT)
    *   `reference_number` (VARCHAR, Nullable) -- e.g., check number, invoice number.
    *   `contact_id` (UUID, Nullable, Foreign Key to `contacts.id`)
    *   `company_id` (UUID, Nullable, Foreign Key to `companies.id`)
    *   `journal_type` (VARCHAR or ENUM, Nullable) -- e.g., 'General', 'Sales', 'Cash Receipts', 'Purchases'
    *   `status` (VARCHAR or ENUM, Default 'draft') -- 'draft', 'posted', 'void'
    *   `created_by` (UUID, Foreign Key to `users.id`)
    *   `created_at` (TIMESTAMPTZ, Not Null, Default NOW())
    *   `updated_at` (TIMESTAMPTZ, Not Null, Default NOW())

*   **`transaction_line_items`** (The heart of double-entry)
    *   `id` (UUID, Primary Key)
    *   `transaction_id` (UUID, Not Null, Foreign Key to `transactions.id` ON DELETE CASCADE)
    *   `account_id` (UUID, Not Null, Foreign Key to `chart_of_accounts.id`)
    *   `description` (TEXT)
    *   `debit_amount` (DECIMAL(15,2), Not Null, Default 0) -- Positive values for debits.
    *   `credit_amount` (DECIMAL(15,2), Not Null, Default 0) -- Positive values for credits.
    *   `created_at` (TIMESTAMPTZ, Not Null, Default NOW())
    *   `updated_at` (TIMESTAMPTZ, Not Null, Default NOW())
    *   Check constraint: `debit_amount * credit_amount = 0` (ensures only one is non-zero).
    *   The sum of `debit_amount` for all line items in a transaction must equal the sum of `credit_amount`. This should be enforced at the application level and potentially via database triggers or procedures if feasible, though application-level enforcement is more common with ORMs/toolkits like SQLx.

### 3.3. Modules (e.g., Invoicing, Expenses)

*   **`invoices`**
    *   `id` (UUID, Primary Key)
    *   `invoice_number` (VARCHAR, Unique, Not Null)
    *   `customer_id` (UUID, Not Null, Foreign Key to `contacts.id` where contact_type = 'Customer')
    *   `invoice_date` (DATE, Not Null)
    *   `due_date` (DATE, Not Null)
    *   `total_amount` (DECIMAL(15,2), Not Null) -- Calculated from line items.
    *   `status` (VARCHAR or ENUM, Not Null) -- 'draft', 'sent', 'paid', 'overdue', 'void'
    *   `company_id` (UUID, Nullable, Foreign Key to `companies.id`)
    *   `created_by` (UUID, Foreign Key to `users.id`)
    *   `created_at` (TIMESTAMPTZ, Not Null, Default NOW())
    *   `updated_at` (TIMESTAMPTZ, Not Null, Default NOW())
    *   When an invoice is "posted", it should generate corresponding entries in the `transactions` and `transaction_line_items` tables (e.g., Debit Accounts Receivable, Credit Revenue).

*   **`invoice_line_items`**
    *   `id` (UUID, Primary Key)
    *   `invoice_id` (UUID, Not Null, Foreign Key to `invoices.id` ON DELETE CASCADE)
    *   `item_description` (TEXT, Not Null)
    *   `quantity` (DECIMAL(10,3), Not Null)
    *   `unit_price` (DECIMAL(15,2), Not Null)
    *   `amount` (DECIMAL(15,2), Not Null) -- quantity * unit_price
    *   `revenue_account_id` (UUID, Not Null, Foreign Key to `chart_of_accounts.id` where account_type = 'Revenue')
    *   `created_at` (TIMESTAMPTZ, Not Null, Default NOW())
    *   `updated_at` (TIMESTAMPTZ, Not Null, Default NOW())

*   **`payments`**
    *   `id` (UUID, Primary Key)
    *   `customer_id` (UUID, Not Null, Foreign Key to `contacts.id` where contact_type = 'Customer')
    *   `payment_date` (DATE, Not Null)
    *   `amount` (DECIMAL(15,2), Not Null)
    *   `payment_method` (VARCHAR or ENUM, Not Null) -- 'Cash', 'Check', 'Credit Card', 'Bank Transfer'
    *   `reference_number` (VARCHAR, Nullable) -- e.g., check number, transaction ID.
    *   `company_id` (UUID, Nullable, Foreign Key to `companies.id`)
    *   `created_by` (UUID, Foreign Key to `users.id`)
    *   `created_at` (TIMESTAMPTZ, Not Null, Default NOW())
    *   `updated_at` (TIMESTAMPTZ, Not Null, Default NOW())
    *   When a payment is recorded, it should generate corresponding entries in the `transactions` and `transaction_line_items` tables (e.g., Debit Cash/Bank, Credit Accounts Receivable). A linking table might be needed to associate payments with specific invoices.

*   **`expenses`**
    *   `id` (UUID, Primary Key)
    *   `vendor_id` (UUID, Nullable, Foreign Key to `contacts.id` where contact_type = 'Vendor')
    *   `expense_date` (DATE, Not Null)
    *   `description` (TEXT, Not Null)
    *   `amount` (DECIMAL(15,2), Not Null)
    *   `expense_account_id` (UUID, Not Null, Foreign Key to `chart_of_accounts.id` where account_type = 'Expense')
    *   `status` (VARCHAR or ENUM, Not Null) -- 'pending', 'approved', 'paid'
    *   `company_id` (UUID, Nullable, Foreign Key to `companies.id`)
    *   `created_by` (UUID, Foreign Key to `users.id`)
    *   `created_at` (TIMESTAMPTZ, Not Null, Default NOW())
    *   `updated_at` (TIMESTAMPTZ, Not Null, Default NOW())
    *   When an expense is "posted" or "paid", it should generate corresponding entries in the `transactions` and `transaction_line_items` tables (e.g., Debit Expense Account, Credit Accounts Payable or Cash).

### 3.4. Indexing and Performance Considerations
*   **Primary Keys:** All primary keys (`id`) will be UUIDs. While sequential integers can offer some performance benefits in specific B-tree scenarios, UUIDs prevent exposing data counts and are often preferred for distributed systems or when unique IDs across environments are needed. PostgreSQL's handling of UUIDs is efficient.
*   **Foreign Keys:** All foreign key columns should be indexed to speed up JOIN operations.
*   **Frequently Queried Columns:** Columns often used in `WHERE` clauses (e.g., `transaction_date` in `transactions`, `invoice_number` in `invoices`, `status` in various tables) should be indexed.
*   **Composite Indexes:** For queries that filter on multiple columns simultaneously (e.g., `company_id` and `transaction_date`), composite indexes can significantly improve performance.
*   **Partial Indexes:** For indexes that are only useful for a subset of data (e.g., only active accounts), partial indexes can save space and improve performance.
*   **Database Vacuuming and Analyzing:** Regular `VACUUM` and `ANALYZE` operations are crucial in PostgreSQL to maintain table health and provide the query planner with up-to-date statistics for optimal query execution.
*   **Connection Pooling:** SQLx provides connection pooling, which is essential for managing database connections efficiently in a web application.

## 4. API Design

The backend will expose a RESTful API for the frontend to interact with. All API endpoints should be secured, requiring appropriate authentication and authorization. Data will primarily be exchanged in JSON format.

### 4.1. General Principles
*   **Versioning:** The API should be versioned (e.g., `/api/v1/...`) to allow for future changes without breaking existing clients.
*   **Authentication:** Token-based authentication (e.g., JWT - JSON Web Tokens) is recommended for stateless authentication. The frontend will include the token in the `Authorization` header of each request.
*   **Authorization:** Role-based access control will be implemented on the backend to ensure users can only access data and perform actions permitted by their assigned roles.
*   **Error Handling:** API responses should include clear and consistent error messages, along with appropriate HTTP status codes (e.g., 400 for Bad Request, 401 for Unauthorized, 403 for Forbidden, 404 for Not Found, 500 for Internal Server Error).
*   **Input Validation:** All incoming data should be rigorously validated on the backend before processing. Rust's type system and libraries like `validator` can help enforce this.

### 4.2. Example API Endpoints

| Resource | HTTP Method | Endpoint | Description | Example Request Body (JSON) |
| :--- | :--- | :--- | :--- | :--- |
| **Authentication** | POST | `/api/v1/auth/login` | User login | `{"username": "user", "password": "pass"}` |
| | POST | `/api/v1/auth/logout` | User logout | |
| | POST | `/api/v1/auth/refresh` | Refresh JWT token | `{"refresh_token": "..."}` |
| **Chart of Accounts** | GET | `/api/v1/accounts` | List all accounts | |
| | POST | `/api/v1/accounts` | Create a new account | `{"code": "1010", "name": "Petty Cash", "account_type": "Asset"}` |
| | GET | `/api/v1/accounts/{id}` | Get a specific account | |
| | PUT | `/api/v1/accounts/{id}` | Update an account | `{"name": "Office Supplies", "is_active": false}` |
| | DELETE | `/api/v1/accounts/{id}` | Delete an account (soft delete preferred) | |
| **Transactions** | GET | `/api/v1/transactions` | List transactions (with filters: date_range, contact_id, etc.) | |
| | POST | `/api/v1/transactions` | Create a new transaction | `{"transaction_date": "2024-07-20", "description": "Initial deposit", "line_items": [{"account_id": "uuid1", "debit_amount": 1000.00}, {"account_id": "uuid2", "credit_amount": 1000.00}]}` |
| | GET | `/api/v1/transactions/{id}` | Get a specific transaction | |
| | PUT | `/api/v1/transactions/{id}` | Update a transaction (if in 'draft' status) | |
| | DELETE | `/api/v1/transactions/{id}` | Void or delete a transaction | |
| **Invoices** | GET | `/api/v1/invoices` | List invoices | |
| | POST | `/api/v1/invoices` | Create a new invoice | `{"customer_id": "uuid", "invoice_date": "2024-07-20", "due_date": "2024-08-20", "line_items": [{"item_description": "Consulting", "quantity": 10, "unit_price": 150.00, "revenue_account_id": "uuid3"}]}` |
| | GET | `/api/v1/invoices/{id}` | Get a specific invoice | |
| | PUT | `/api/v1/invoices/{id}` | Update an invoice (if in 'draft' status) | |
| | DELETE | `/api/v1/invoices/{id}` | Void or delete an invoice | |
| | POST | `/api/v1/invoices/{id}/send` | Mark invoice as 'sent' (might trigger email notification) | |
| **Reports** | GET | `/api/v1/reports/profit-loss` | Generate Profit & Loss report | Query params: `start_date`, `end_date` |
| | GET | `/api/v1/reports/balance-sheet` | Generate Balance Sheet report | Query params: `as_of_date` |
| | GET | `/api/v1/reports/trial-balance` | Generate Trial Balance report | Query params: `as_of_date` |

### 4.3. Authentication and Authorization Strategy
*   **Authentication:** JWTs will be used. Upon successful login, the backend will issue an access token (with a short expiry, e.g., 15-30 minutes) and a refresh token (with a longer expiry, e.g., 7 days, stored securely). The frontend will send the access token in the `Authorization: Bearer <token>` header with each API call. When the access token expires, the refresh token can be used to obtain a new access token without requiring the user to log in again.
*   **Authorization:** User roles (e.g., 'admin', 'accountant', 'viewer') will be defined. API endpoints or specific actions within endpoints will be protected based on these roles. For instance, only users with the 'admin' role might be able to manage user accounts or modify the chart of accounts, while 'accountant' roles can create and post transactions. This logic will be implemented in middleware or route handlers within the Actix Web/Axum application.

## 5. Data Migration Strategy from QuickBooks

Migrating existing data from QuickBooks is a critical and sensitive task. The strategy must prioritize data integrity and accuracy.

### 5.1. Data Export from QuickBooks
*   **Method:** QuickBooks Desktop allows exporting various lists and reports to CSV or Excel formats [[30](https://quickbooks.intuit.com/learn-support/en-us/help-article/manage-lists/import-export-csv-files/L9AiGRdT9_US_en_US)], [[31](https://quickbooks.intuit.com/learn-support/en-us/help-article/import-export-data-files/import-export-data-quickbooks-desktop/L9KS42UxP_US_en_US)]. For QuickBooks Online, similar export functionalities are available.
*   **Data to Export:** The following data entities will need to be exported:
    *   Chart of Accounts
    *   Customer List
    *   Vendor List
    *   Item List (if applicable)
    *   Opening Balances (if available as a report)
    *   Invoices (or Sales Receipts)
    *   Customer Payments
    *   Bills (Vendor Invoices)
    *   Bill Payments (Vendor Credits/Checks)
    *   Journal Entries (for any adjustments)
    *   General Ledger detail (if individual transaction export is not granular enough)
*   **Challenges:** The granularity and structure of exported data might not perfectly align with the new schema. For example, QuickBooks might export invoices and payments separately, and their relationships might need to be reconstructed. Some data fields might be missing or formatted differently.

### 5.2. Data Transformation and Import Scripts
*   **Language:** Rust will be used to write command-line utilities or dedicated migration modules within the application. Rust's performance and strong typing make it well-suited for data processing tasks.
*   **Libraries:** The `csv` crate in Rust is excellent for parsing CSV files. `serde` will be used for deserializing data into Rust structs.
*   **Process:**
    1.  **Parse Exported Files:** Rust scripts will read the CSV/Excel files exported from QuickBooks.
    2.  **Validate Data:** Each record will be validated for required fields, data types, and basic business logic.
    3.  **Transform Data:** The parsed data will be transformed to match the schema of Project LedgerForge's database. This involves mapping QuickBooks field names to new field names, converting data formats, and restructuring relationships (e.g., linking invoice line items to their parent invoice).
    4.  **Handle Data Inconsistencies:** QuickBooks data might have inconsistencies (e.g., orphaned records, missing references). The scripts should be robust enough to log these issues and potentially skip problematic records or apply default values where appropriate, with careful review.
    5.  **Import into PostgreSQL:** Using SQLx, the transformed and validated data will be inserted into the corresponding tables in the PostgreSQL database.
    6.  **Maintain Referential Integrity:** The import order is crucial. For example, `chart_of_accounts` and `contacts` should be imported before `transactions` or `invoices`. The scripts must ensure that foreign key relationships are correctly established.
    7.  **Generate Core Transactions:** For data like invoices and expenses imported from QuickBooks, the migration scripts will need to generate the corresponding entries in the `transactions` and `transaction_line_items` tables to maintain the integrity of the double-entry system within Project LedgerForge. This might involve reverse-engineering the debit/credit entries based on the QuickBooks data (e.g., an invoice implies a debit to A/R and credit to Revenue).

### 5.3. Data Validation and Reconciliation
*   **Trial Balance:** After the import is complete, a trial balance report should be generated from Project LedgerForge as of the migration cutoff date. This trial balance must be meticulously compared against a trial balance from QuickBooks for the same date. Any discrepancies must be investigated and resolved.
*   **Aging Reports:** Customer and vendor aging reports should also be compared between the old and new systems.
*   **Spot Checks:** Randomly select specific customers, vendors, invoices, and payments and verify that their details and outstanding balances are correct in the new system.
*   **User Acceptance Testing (UAT) for Migrated Data:** Involve accounting staff to review the migrated data and perform their usual tasks in the new system to identify any issues.

## 6. Development Roadmap

A phased approach will be adopted to manage the project effectively and deliver value incrementally.

### 6.1. Phase 1: Foundation and Core Engine (4-6 weeks)
*   **Goal:** Establish the project structure, set up the development environment, and implement the core double-entry accounting engine.
*   **Tasks:**
    *   Initialize Rust project, choose and configure backend framework (Actix Web/Axum).
    *   Set up PostgreSQL database and configure SQLx.
    *   Design and implement database migrations for core tables: `users`, `chart_of_accounts`, `contacts`, `transactions`, `transaction_line_items`.
    *   Implement basic user authentication (login, JWT token generation).
    *   Develop API endpoints for CRUD operations on the chart of accounts.
    *   Develop API endpoints for creating, reading, and listing transactions, ensuring double-entry logic is enforced in the backend.
    *   Implement basic authorization (e.g., differentiating between authenticated users).
*   **Deliverable:** A backend capable of managing users, accounts, and core double-entry transactions via API calls.

### 6.2. Phase 2: Frontend Integration and Key Modules (6-8 weeks)
*   **Goal:** Develop the frontend application and integrate it with the backend. Implement key accounting modules like invoicing and expense tracking.
*   **Tasks:**
    *   Set up the Rust frontend project (Leptos/Dioxus).
    *   Implement user login and registration UI.
    *   Create UI for managing the chart of accounts.
    *   Develop UI for transaction entry, ensuring it guides the user to create balanced entries.
    *   Implement the invoicing module: UI for creating, viewing, and managing invoices. Backend logic to generate corresponding transactions when invoices are finalized.
    *   Implement the expense tracking module: UI for recording and managing expenses. Backend logic to generate corresponding transactions.
    *   Implement basic customer and vendor management UI.
*   **Deliverable:** A functional web application where users can log in, manage their chart of accounts, create invoices, record expenses, and enter general journal transactions.

### 6.3. Phase 3: Data Migration and Reporting (4-6 weeks)
*   **Goal:** Develop the data migration tools and core financial reporting capabilities.
*   **Tasks:**
    *   Analyze QuickBooks export formats in detail.
    *   Develop Rust scripts for exporting data from QuickBooks (manual export by user, then script processes the file).
    *   Develop Rust scripts for transforming and importing data into the Project LedgerForge database.
    *   Implement core financial reports: Profit & Loss, Balance Sheet, Trial Balance. These can be generated on the backend and displayed on the frontend, potentially with options to export as PDF or CSV.
    *   Conduct a full data migration using test data from QuickBooks and perform thorough validation and reconciliation.
*   **Deliverable:** An application capable of migrating QuickBooks data and generating essential financial reports.

### 6.4. Phase 4: Advanced Features, Polish, and Deployment (4-6 weeks)
*   **Goal:** Add remaining features, refine the UI/UX, and prepare for production deployment.
*   **Tasks:**
    *   Implement bank reconciliation module.
    *   Add features like user role management, audit logging for critical changes.
    *   Enhance reports with more filtering, customization, and comparison options.
    *   Polish the frontend UI/UX based on feedback.
    *   Set up CI/CD pipeline.
    *   Perform comprehensive end-to-end testing (UAT).
    *   Prepare production environment and deployment scripts.
    *   Conduct final data migration from the live QuickBooks instance.
    *   Deploy Project LedgerForge to production.
    *   Provide training to end-users.
*   **Deliverable:** A production-ready, custom accounting system with migrated data and trained users.

### 6.5. Phase 5: Post-Launch Support and Iteration (Ongoing)
*   **Goal:** Ensure system stability, address user feedback, and plan for future enhancements.
*   **Tasks:**
    *   Monitor system performance and security.
    *   Provide ongoing support and bug fixes.
    *   Gather user feedback for improvements.
    *   Plan and develop new features in iterative cycles (e.g., payroll, advanced inventory, more sophisticated analytics).

*Estimated total time for Phases 1-4: 20-26 weeks, assuming a dedicated developer or small team. This is a rough estimate and can vary based on feature complexity and resource availability.*

## 7. Security Considerations

Financial data is highly sensitive, and security must be a paramount concern throughout the development lifecycle.

### 7.1. Data Security
*   **Encryption in Transit:** All communication between the client and server must be encrypted using TLS/SSL (HTTPS).
*   **Encryption at Rest:** Sensitive data in the database (e.g., passwords, potentially specific PII) should be encrypted. PostgreSQL offers pgcrypto for data encryption. Full disk encryption on the database server is also recommended.
*   **Secure Password Storage:** User passwords must be hashed using a strong, adaptive hashing algorithm like Argon2 or bcrypt. Never store plaintext passwords.
*   **Database Access Control:** Configure PostgreSQL user roles and permissions to restrict database access. The application should connect using a least-privilege database user.
*   **Regular Backups:** Implement automated, regular backups of the database. Test restoration procedures periodically. Store backups securely.

### 7.2. Application Security
*   **Input Validation:** Rigorously validate all user input on both the client and server sides to prevent injection attacks (SQL injection, XSS, etc.). Rust's type system and libraries like `validator` are helpful here.
*   **Authentication and Authorization:** Implement robust JWT-based authentication and enforce role-based access control for all API endpoints.
*   **Secure Session Management:** If server-side sessions are used for any reason (though JWT aims for statelessness), ensure secure session handling.
*   **Protection Against Common Vulnerabilities:** Stay informed about common web vulnerabilities (OWASP Top 10) and ensure the application is protected. Frameworks like Actix Web and Axum often provide built-in protections against many of these.
*   **Dependency Management:** Regularly update Rust crates and other dependencies to patch known security vulnerabilities. Tools like `cargo-audit` can help.
*   **Logging and Monitoring:** Implement comprehensive logging for security-relevant events (failed logins, access denied errors, critical data changes). Monitor logs for suspicious activity.
*   **Error Handling:** Avoid exposing sensitive information in error messages sent to the client. Log detailed errors server-side for debugging.

### 7.3. Rust-Specific Security Advantages
*   **Memory Safety:** Rust's ownership and borrowing system prevent entire classes of memory-related vulnerabilities common in C/C++, such as buffer overflows, use-after-free, and data races, at compile time. This is a significant advantage for building secure systems.
*   **Type Safety:** Rust's strong static type system helps catch many programming errors at compile time, reducing the potential for runtime bugs that could have security implications.

## 8. Testing Strategy

A comprehensive testing strategy is essential to ensure the reliability and correctness of the accounting system.

### 8.1. Unit Testing
*   **Focus:** Test individual functions, methods, and structs in isolation.
*   **Tools:** Rust's built-in testing framework.
*   **Examples:**
    *   Test that a function calculating transaction totals works correctly.
    *   Test that a data validation function correctly identifies valid and invalid inputs.
    *   Test that specific business logic (e.g., applying a payment to an invoice) updates the correct fields.
    *   Aim for high code coverage for core logic, especially around financial calculations.

### 8.2. Integration Testing
*   **Focus:** Test the interaction between different modules or components of the application.
*   **Tools:** Rust's built-in testing framework can be used for integration tests, often involving a test database instance.
*   **Examples:**
    *   Test the full flow from an API request for creating an invoice to the data being correctly persisted in the database, including the generation of corresponding transaction entries.
    *   Test that user authentication and authorization middleware correctly protects API endpoints.
    *   Test the data import process with sample QuickBooks export files.

### 8.3. End-to-End (E2E) Testing / User Acceptance Testing (UAT)
*   **Focus:** Test the entire application from the user's perspective to ensure it meets business requirements and is usable.
*   **Tools:** Browser automation tools like Selenium, Cypress, or potentially Rust-based tools if mature enough for this purpose. However, manual UAT by accounting staff is invaluable.
*   **Examples:**
    *   Simulate a user logging in, creating an invoice, recording a payment, and then verifying that the customer's outstanding balance is updated and that the P&L report reflects the revenue.
    *   Test the data migration process with a copy of the actual QuickBooks data and have accounting staff verify the results in the new system.
    *   Test all major user workflows and reports.

### 8.4. Performance Testing
*   **Focus:** Ensure the application performs well under expected loads.
*   **Tools:** Tools like `wrk`, `k6`, or custom Rust scripts.
*   **Examples:**
    *   Benchmark API endpoints for common operations (e.g., fetching a list of transactions, generating a report).
    *   Simulate multiple concurrent users to identify bottlenecks.
    *   Test report generation with large datasets.

## 9. Deployment and Operations

### 9.1. Environment Setup
*   **Development:** Local development environments will use Docker Compose to spin up the application, PostgreSQL, and any other dependencies.
*   **Staging:** A staging environment that mirrors the production environment will be used for final testing and UAT before each release.
*   **Production:** A secure, scalable environment on a chosen cloud provider.

### 9.2. CI/CD Pipeline
*   **Source Control:** Git (e.g., on GitHub, GitLab).
*   **Automation:**
    *   On every push to the main branch: Run all unit and integration tests, perform linting (`cargo clippy` and `cargo fmt --check`).
    *   On successful tests and merges to main (or a dedicated release branch): Build the application binary and Docker image.
    *   Automatically deploy the new version to the staging environment.
    *   Manual approval required for deployment to production.
*   **Tools:** GitHub Actions, GitLab CI/CD, Jenkins, etc.

### 9.3. Monitoring and Logging
*   **Application Logs:** Structured logging (e.g., using the `tracing` and `tracing-subscriber` crates in Rust) to capture important events, errors, and performance metrics. Logs should be aggregated and searchable.
*   **System Monitoring:** Monitor server health (CPU, memory, disk usage, network traffic).
*   **Application Performance Monitoring (APM):** Consider using APM tools to track request latency, error rates, and database query performance.
*   **Alerting:** Set up alerts for critical errors, high error rates, or unusual system behavior.

## 10. Future Enhancements

The initial release will focus on core accounting functionalities. Future enhancements could include:

*   **Payroll Module:** A comprehensive module for managing employee payroll, tax calculations, and payments.
*   **Advanced Inventory Management:** Tracking stock levels, cost of goods sold, and purchase orders.
*   **Multi-Currency Support:** Handling transactions and reporting in multiple currencies.
*   **Multi-Tenancy:** Allowing multiple companies to use the same instance of the application with data isolation.
*   **Advanced Reporting and Analytics:** More sophisticated dashboards, customizable report builders, and data visualization tools.
*   **API for Third-Party Integrations:** Exposing a richer API for integration with e-commerce platforms, payment gateways, or other business tools.
*   **Mobile Application:** A native or web-based mobile app for on-the-go access.
*   **Artificial Intelligence for Insights:** AI-powered features for anomaly detection, cash flow forecasting, or expense categorization suggestions.

## 11. Conclusion

Project LedgerForge represents a significant but strategically important initiative to build a custom accounting system using the Rust programming language. By leveraging Rust's performance, safety, and modern ecosystem, along with a carefully designed architecture and development process, we aim to create a financial management tool that is not only a replacement for QuickBooks but a superior asset tailored to the specific needs of the business. This design document provides a comprehensive blueprint for this journey, outlining the technical choices, architectural considerations, and implementation roadmap. Success will depend on disciplined execution, rigorous testing, and a commitment to ongoing improvement. The result will be a robust, secure, and highly customized accounting system that empowers the business with greater control, insight, and financial autonomy.