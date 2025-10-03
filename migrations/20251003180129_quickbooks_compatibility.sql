-- QuickBooks Compatibility Enhancement
-- Add fields to support QuickBooks data structure and migration

-- Enhance Chart of Accounts with QuickBooks fields
ALTER TABLE chart_of_accounts
    ADD COLUMN description TEXT,
    ADD COLUMN account_subtype VARCHAR(100),
    ADD COLUMN quickbooks_id VARCHAR(50),
    ADD COLUMN fully_qualified_name VARCHAR(500), -- QB hierarchical name like "Assets:Bank:Checking"
    ADD COLUMN current_balance DECIMAL(15,2) DEFAULT 0,
    ADD COLUMN currency_code VARCHAR(10) DEFAULT 'USD';

-- Enhance Contacts with QuickBooks fields
ALTER TABLE contacts
    ADD COLUMN quickbooks_id VARCHAR(50),
    ADD COLUMN company_name VARCHAR(255),
    ADD COLUMN display_name VARCHAR(255),
    ADD COLUMN active BOOLEAN DEFAULT true,
    ADD COLUMN balance DECIMAL(15,2) DEFAULT 0,
    ADD COLUMN primary_tax_identifier VARCHAR(50), -- SSN/EIN for vendors
    ADD COLUMN website VARCHAR(255),
    ADD COLUMN notes TEXT;

-- Enhance Transactions with QuickBooks fields
ALTER TABLE transactions
    ADD COLUMN quickbooks_id VARCHAR(50),
    ADD COLUMN quickbooks_type VARCHAR(50), -- Invoice, Payment, Bill, etc.
    ADD COLUMN doc_number VARCHAR(100), -- QuickBooks document number
    ADD COLUMN currency_code VARCHAR(10) DEFAULT 'USD',
    ADD COLUMN exchange_rate DECIMAL(10,6) DEFAULT 1.0,
    ADD COLUMN private_note TEXT,
    ADD COLUMN txn_source VARCHAR(50); -- API, UI, Import, etc.

-- Create Invoices table (QuickBooks Invoice entity)
CREATE TABLE invoices (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    quickbooks_id VARCHAR(50),
    invoice_number VARCHAR(100) UNIQUE NOT NULL,
    customer_id UUID NOT NULL REFERENCES contacts(id),
    invoice_date DATE NOT NULL,
    due_date DATE NOT NULL,
    ship_date DATE,
    tracking_number VARCHAR(100),
    total_amount DECIMAL(15,2) NOT NULL DEFAULT 0,
    balance DECIMAL(15,2) NOT NULL DEFAULT 0,
    status VARCHAR(50) NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'sent', 'paid', 'partial', 'overdue', 'void')),
    customer_memo TEXT,
    billing_address TEXT,
    shipping_address TEXT,
    company_id UUID REFERENCES companies(id),
    transaction_id UUID REFERENCES transactions(id), -- Link to generated journal entry
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create Invoice Line Items table
CREATE TABLE invoice_line_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    invoice_id UUID NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    line_number INT NOT NULL,
    item_description TEXT NOT NULL,
    quantity DECIMAL(10,3) NOT NULL DEFAULT 1,
    unit_price DECIMAL(15,2) NOT NULL,
    amount DECIMAL(15,2) NOT NULL, -- quantity * unit_price
    discount_percent DECIMAL(5,2) DEFAULT 0,
    discount_amount DECIMAL(15,2) DEFAULT 0,
    tax_code VARCHAR(50),
    revenue_account_id UUID NOT NULL REFERENCES chart_of_accounts(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create Bills table (QuickBooks Bill entity - vendor invoices)
CREATE TABLE bills (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    quickbooks_id VARCHAR(50),
    bill_number VARCHAR(100),
    vendor_id UUID NOT NULL REFERENCES contacts(id),
    bill_date DATE NOT NULL,
    due_date DATE NOT NULL,
    total_amount DECIMAL(15,2) NOT NULL DEFAULT 0,
    balance DECIMAL(15,2) NOT NULL DEFAULT 0,
    status VARCHAR(50) NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'paid', 'partial', 'void')),
    memo TEXT,
    company_id UUID REFERENCES companies(id),
    transaction_id UUID REFERENCES transactions(id),
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create Bill Line Items table
CREATE TABLE bill_line_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    bill_id UUID NOT NULL REFERENCES bills(id) ON DELETE CASCADE,
    line_number INT NOT NULL,
    description TEXT,
    amount DECIMAL(15,2) NOT NULL,
    expense_account_id UUID NOT NULL REFERENCES chart_of_accounts(id),
    billable BOOLEAN DEFAULT false,
    customer_id UUID REFERENCES contacts(id), -- If billable to customer
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create Payments table (Customer Payments)
CREATE TABLE payments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    quickbooks_id VARCHAR(50),
    payment_number VARCHAR(100),
    customer_id UUID NOT NULL REFERENCES contacts(id),
    payment_date DATE NOT NULL,
    amount DECIMAL(15,2) NOT NULL,
    unapplied_amount DECIMAL(15,2) DEFAULT 0,
    payment_method VARCHAR(50) NOT NULL, -- Cash, Check, Credit Card, Bank Transfer, etc.
    reference_number VARCHAR(100), -- Check number, transaction ID
    deposit_to_account_id UUID REFERENCES chart_of_accounts(id), -- Bank account
    memo TEXT,
    company_id UUID REFERENCES companies(id),
    transaction_id UUID REFERENCES transactions(id),
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create Payment Applications table (links payments to invoices)
CREATE TABLE payment_applications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    payment_id UUID NOT NULL REFERENCES payments(id) ON DELETE CASCADE,
    invoice_id UUID NOT NULL REFERENCES invoices(id),
    amount_applied DECIMAL(15,2) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create Bill Payments table (Vendor Payments)
CREATE TABLE bill_payments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    quickbooks_id VARCHAR(50),
    payment_number VARCHAR(100),
    vendor_id UUID NOT NULL REFERENCES contacts(id),
    payment_date DATE NOT NULL,
    amount DECIMAL(15,2) NOT NULL,
    payment_method VARCHAR(50) NOT NULL,
    reference_number VARCHAR(100),
    bank_account_id UUID REFERENCES chart_of_accounts(id),
    memo TEXT,
    company_id UUID REFERENCES companies(id),
    transaction_id UUID REFERENCES transactions(id),
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create Bill Payment Applications table
CREATE TABLE bill_payment_applications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    bill_payment_id UUID NOT NULL REFERENCES bill_payments(id) ON DELETE CASCADE,
    bill_id UUID NOT NULL REFERENCES bills(id),
    amount_applied DECIMAL(15,2) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create Items/Products table (QuickBooks Items)
CREATE TABLE items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    quickbooks_id VARCHAR(50),
    name VARCHAR(255) NOT NULL,
    sku VARCHAR(100),
    item_type VARCHAR(50) NOT NULL CHECK (item_type IN ('Service', 'Inventory', 'Non-Inventory')),
    description TEXT,
    unit_price DECIMAL(15,2),
    purchase_cost DECIMAL(15,2),
    quantity_on_hand DECIMAL(10,3) DEFAULT 0,
    income_account_id UUID REFERENCES chart_of_accounts(id),
    expense_account_id UUID REFERENCES chart_of_accounts(id),
    asset_account_id UUID REFERENCES chart_of_accounts(id), -- For inventory
    active BOOLEAN DEFAULT true,
    taxable BOOLEAN DEFAULT false,
    company_id UUID REFERENCES companies(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Add indexes for QuickBooks fields
CREATE INDEX idx_accounts_qb_id ON chart_of_accounts(quickbooks_id);
CREATE INDEX idx_contacts_qb_id ON contacts(quickbooks_id);
CREATE INDEX idx_contacts_display_name ON contacts(display_name);
CREATE INDEX idx_transactions_qb_id ON transactions(quickbooks_id);
CREATE INDEX idx_transactions_qb_type ON transactions(quickbooks_type);
CREATE INDEX idx_invoices_qb_id ON invoices(quickbooks_id);
CREATE INDEX idx_invoices_number ON invoices(invoice_number);
CREATE INDEX idx_invoices_customer ON invoices(customer_id);
CREATE INDEX idx_invoices_status ON invoices(status);
CREATE INDEX idx_bills_vendor ON bills(vendor_id);
CREATE INDEX idx_payments_customer ON payments(customer_id);
CREATE INDEX idx_bill_payments_vendor ON bill_payments(vendor_id);
CREATE INDEX idx_items_type ON items(item_type);

-- Triggers for updated_at on new tables
CREATE TRIGGER update_invoices_updated_at BEFORE UPDATE ON invoices
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_invoice_line_items_updated_at BEFORE UPDATE ON invoice_line_items
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_bills_updated_at BEFORE UPDATE ON bills
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_bill_line_items_updated_at BEFORE UPDATE ON bill_line_items
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_payments_updated_at BEFORE UPDATE ON payments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_bill_payments_updated_at BEFORE UPDATE ON bill_payments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_items_updated_at BEFORE UPDATE ON items
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
