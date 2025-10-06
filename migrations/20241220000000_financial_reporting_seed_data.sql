-- Comprehensive seed data for financial reporting tests
-- This script creates realistic test data to validate all reporting endpoints

-- Clear existing data while preserving structure
TRUNCATE TABLE transaction_line_items, transactions, contacts, chart_of_accounts, companies, users RESTART IDENTITY CASCADE;

-- Create test company
INSERT INTO companies (id, name, address) VALUES
('550e8400-e29b-41d4-a716-446655440001', 'Test Corporation Inc.', '123 Business St, Test City, TC 12345');

-- Create test users
INSERT INTO users (id, username, email, password_hash, role) VALUES
('550e8400-e29b-41d4-a716-446655440010', 'admin', 'admin@testcorp.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewKyNjrKK9H9Q7Oa', 'admin'),
('550e8400-e29b-41d4-a716-446655440011', 'accountant', 'accountant@testcorp.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewKyNjrKK9H9Q7Oa', 'accountant');

-- Create comprehensive chart of accounts
INSERT INTO chart_of_accounts (id, code, name, account_type, company_id) VALUES
-- Assets
('550e8400-e29b-41d4-a716-446655440100', '1000', 'Cash and Cash Equivalents', 'Asset', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440101', '1010', 'Business Checking Account', 'Asset', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440102', '1100', 'Accounts Receivable', 'Asset', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440103', '1200', 'Inventory', 'Asset', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440104', '1300', 'Prepaid Expenses', 'Asset', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440105', '1500', 'Office Equipment', 'Asset', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440106', '1510', 'Accumulated Depreciation - Office Equipment', 'Asset', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440107', '1600', 'Computer Equipment', 'Asset', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440108', '1610', 'Accumulated Depreciation - Computer Equipment', 'Asset', '550e8400-e29b-41d4-a716-446655440001'),

-- Liabilities
('550e8400-e29b-41d4-a716-446655440200', '2000', 'Accounts Payable', 'Liability', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440201', '2100', 'Accrued Expenses', 'Liability', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440202', '2200', 'Taxes Payable', 'Liability', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440203', '2300', 'Short-term Debt', 'Liability', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440204', '2500', 'Long-term Debt', 'Liability', '550e8400-e29b-41d4-a716-446655440001'),

-- Equity
('550e8400-e29b-41d4-a716-446655440300', '3000', 'Owner''s Capital', 'Equity', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440301', '3100', 'Retained Earnings', 'Equity', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440302', '3200', 'Common Stock', 'Equity', '550e8400-e29b-41d4-a716-446655440001'),

-- Revenue
('550e8400-e29b-41d4-a716-446655440400', '4000', 'Sales Revenue', 'Revenue', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440401', '4010', 'Product Sales', 'Revenue', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440402', '4020', 'Service Revenue', 'Revenue', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440403', '4030', 'Consulting Revenue', 'Revenue', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440404', '4040', 'Interest Income', 'Revenue', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440405', '4050', 'Discounts Allowed', 'Revenue', '550e8400-e29b-41d4-a716-446655440001'),

-- Expenses
('550e8400-e29b-41d4-a716-446655440500', '5000', 'Cost of Goods Sold', 'Expense', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440501', '5010', 'Materials Cost', 'Expense', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440502', '5020', 'Labor Cost', 'Expense', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440503', '6000', 'Operating Expenses', 'Expense', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440504', '6010', 'Office Rent', 'Expense', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440505', '6020', 'Utilities', 'Expense', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440506', '6030', 'Office Supplies', 'Expense', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440507', '6040', 'Marketing Expenses', 'Expense', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440508', '6050', 'Travel Expenses', 'Expense', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440509', '6060', 'Professional Services', 'Expense', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440510', '6070', 'Insurance', 'Expense', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440511', '6080', 'Depreciation Expense', 'Expense', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440512', '6090', 'Payroll Expenses', 'Expense', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440513', '6100', 'Bank Fees', 'Expense', '550e8400-e29b-41d4-a716-446655440001');

-- Create test contacts (customers and vendors)
INSERT INTO contacts (id, contact_type, name, email, phone, billing_address, company_id) VALUES
-- Customers
('550e8400-e29b-41d4-a716-446655440600', 'Customer', 'ABC Manufacturing', 'billing@abcman.com', '555-0101', '456 Industrial Ave, Factory City, FC 67890', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440601', 'Customer', 'XYZ Services LLC', 'accounts@xyzservices.com', '555-0102', '789 Service Rd, Consulting Town, CT 23456', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440602', 'Customer', 'Global Tech Corp', 'ap@globaltech.com', '555-0103', '321 Innovation Blvd, Tech City, TC 54321', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440603', 'Customer', 'Retail Solutions Inc', 'finance@retailsolutions.com', '555-0104', '654 Market St, Retail City, RC 87654', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440604', 'Customer', 'StartUp Ventures', 'founder@startupventures.com', '555-0105', '987 Entrepreneur Way, Innovation City, IC 13579', '550e8400-e29b-41d4-a716-446655440001'),

-- Vendors
('550e8400-e29b-41d4-a716-446655440610', 'Vendor', 'Office Supply Co', 'sales@officesupply.com', '555-0201', '111 Stationery Rd, Paper Town, PT 11111', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440611', 'Vendor', 'Software Solutions Ltd', 'support@softwaresolutions.com', '555-0202', '222 Code Lane, Dev City, DC 22222', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440612', 'Vendor', 'Clean & Janitorial Services', 'billing@cleanjan.com', '555-0203', '333 Clean Blvd, Sanitary City, SC 33333', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440613', 'Vendor', 'Legal Eagles Firm', 'clients@legaleagles.com', '555-0204', '444 Court St, Legal City, LC 44444', '550e8400-e29b-41d4-a716-446655440001'),
('550e8400-e29b-41d4-a716-446655440614', 'Vendor', 'Utility Providers Inc', 'service@utility.com', '555-0205', '555 Power Ave, Energy City, EC 55555', '550e8400-e29b-41d4-a716-446655440001');

-- Create realistic transactions spanning different dates and scenarios
-- Initial capital investment
INSERT INTO transactions (id, transaction_date, description, reference_number, company_id, journal_type, status, created_by) VALUES
('550e8400-e29b-41d4-a716-446655440700', '2024-01-01', 'Initial capital investment', 'INV-001', '550e8400-e29b-41d4-a716-446655440001', 'General', 'posted', '550e8400-e29b-41d4-a716-446655440010');

INSERT INTO transaction_line_items (id, transaction_id, account_id, description, debit_amount, credit_amount) VALUES
('550e8400-e29b-41d4-a716-446655440701', '550e8400-e29b-41d4-a716-446655440700', '550e8400-e29b-41d4-a716-446655440101', 'Initial cash deposit', 100000.00, 0.00),
('550e8400-e29b-41d4-a716-446655440702', '550e8400-e29b-41d4-a716-446655440700', '550e8400-e29b-41d4-a716-446655440300', 'Owner''s capital contribution', 0.00, 100000.00);

-- Purchase office equipment
INSERT INTO transactions (id, transaction_date, description, reference_number, contact_id, company_id, journal_type, status, created_by) VALUES
('550e8400-e29b-41d4-a716-446655440710', '2024-01-05', 'Purchase office computers and furniture', 'PO-001', '550e8400-e29b-41d4-a716-446655440611', 'Purchases', 'posted', '550e8400-e29b-41d4-a716-446655440010');

INSERT INTO transaction_line_items (id, transaction_id, account_id, description, debit_amount, credit_amount) VALUES
('550e8400-e29b-41d4-a716-446655440711', '550e8400-e29b-41d4-a716-446655440710', '550e8400-e29b-41d4-a716-446655440107', 'Computer equipment purchase', 15000.00, 0.00),
('550e8400-e29b-41d4-a716-446655440712', '550e8400-e29b-41d4-a716-446655440710', '550e8400-e29b-41d4-a716-446655440101', 'Payment for computer equipment', 0.00, 15000.00);

-- Monthly rent payment for January
INSERT INTO transactions (id, transaction_date, description, reference_number, contact_id, company_id, journal_type, status, created_by) VALUES
('550e8400-e29b-41d4-a716-446655440720', '2024-01-15', 'January office rent', 'RENT-2024-01', '550e8400-e29b-41d4-a716-446655440610', 'General', 'posted', '550e8400-e29b-41d4-a716-446655440010');

INSERT INTO transaction_line_items (id, transaction_id, account_id, description, debit_amount, credit_amount) VALUES
('550e8400-e29b-41d4-a716-446655440721', '550e8400-e29b-41d4-a716-446655440720', '550e8400-e29b-41d4-a716-446655440504', 'January office rent expense', 2500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440722', '550e8400-e29b-41d4-a716-446655440720', '550e8400-e29b-41d4-a716-446655440101', 'Rent payment', 0.00, 2500.00);

-- Sales to customers throughout the year
INSERT INTO transactions (id, transaction_date, description, reference_number, contact_id, company_id, journal_type, status, created_by) VALUES
('550e8400-e29b-41d4-a716-446655440730', '2024-01-20', 'Product sales to ABC Manufacturing', 'INV-1001', '550e8400-e29b-41d4-a716-446655440600', 'Sales', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440731', '2024-02-15', 'Service revenue from XYZ Services', 'INV-1002', '550e8400-e29b-41d4-a716-446655440601', 'Sales', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440732', '2024-03-10', 'Consulting project for Global Tech', 'INV-1003', '550e8400-e29b-41d4-a716-446655440602', 'Sales', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440733', '2024-04-05', 'Product sales to Retail Solutions', 'INV-1004', '550e8400-e29b-41d4-a716-446655440603', 'Sales', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440734', '2024-05-12', 'Service revenue from StartUp Ventures', 'INV-1005', '550e8400-e29b-41d4-a716-446655440604', 'Sales', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440735', '2024-06-18', 'Product sales to ABC Manufacturing', 'INV-1006', '550e8400-e29b-41d4-a716-446655440600', 'Sales', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440736', '2024-07-22', 'Consulting for Global Tech Corp', 'INV-1007', '550e8400-e29b-41d4-a716-446655440602', 'Sales', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440737', '2024-08-14', 'Service revenue XYZ Services', 'INV-1008', '550e8400-e29b-41d4-a716-446655440601', 'Sales', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440738', '2024-09-25', 'Product sales to Retail Solutions', 'INV-1009', '550e8400-e29b-41d4-a716-446655440603', 'Sales', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440739', '2024-10-30', 'StartUp Ventures project completion', 'INV-1010', '550e8400-e29b-41d4-a716-446655440604', 'Sales', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440740', '2024-11-15', 'ABC Manufacturing bulk order', 'INV-1011', '550e8400-e29b-41d4-a716-446655440600', 'Sales', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440741', '2024-12-20', 'Global Tech year-end project', 'INV-1012', '550e8400-e29b-41d4-a716-446655440602', 'Sales', 'posted', '550e8400-e29b-41d4-a716-446655440010');

-- Sales transaction line items (Revenue and Accounts Receivable)
INSERT INTO transaction_line_items (id, transaction_id, account_id, description, debit_amount, credit_amount) VALUES
-- January sales
('550e8400-e29b-41d4-a716-446655440750', '550e8400-e29b-41d4-a716-446655440730', '550e8400-e29b-41d4-a716-446655440102', 'ABC Manufacturing - Product sales', 12500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440751', '550e8400-e29b-41d4-a716-446655440730', '550e8400-e29b-41d4-a716-446655440401', 'ABC Manufacturing - Product revenue', 0.00, 12500.00),

-- February sales
('550e8400-e29b-41d4-a716-446655440752', '550e8400-e29b-41d4-a716-446655440731', '550e8400-e29b-41d4-a716-446655440102', 'XYZ Services - Service revenue', 8500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440753', '550e8400-e29b-41d4-a716-446655440731', '550e8400-e29b-41d4-a716-446655440402', 'XYZ Services - Service revenue', 0.00, 8500.00),

-- March sales
('550e8400-e29b-41d4-a716-446655440754', '550e8400-e29b-41d4-a716-446655440732', '550e8400-e29b-41d4-a716-446655440102', 'Global Tech - Consulting revenue', 15000.00, 0.00),
('550e8400-e29b-41d4-a716-446655440755', '550e8400-e29b-41d4-a716-446655440732', '550e8400-e29b-41d4-a716-446655440403', 'Global Tech - Consulting revenue', 0.00, 15000.00),

-- April sales
('550e8400-e29b-41d4-a716-446655440756', '550e8400-e29b-41d4-a716-446655440733', '550e8400-e29b-41d4-a716-446655440102', 'Retail Solutions - Product sales', 9800.00, 0.00),
('550e8400-e29b-41d4-a716-446655440757', '550e8400-e29b-41d4-a716-446655440733', '550e8400-e29b-41d4-a716-446655440401', 'Retail Solutions - Product revenue', 0.00, 9800.00),

-- May sales
('550e8400-e29b-41d4-a716-446655440758', '550e8400-e29b-41d4-a716-446655440734', '550e8400-e29b-41d4-a716-446655440102', 'StartUp Ventures - Service revenue', 7200.00, 0.00),
('550e8400-e29b-41d4-a716-446655440759', '550e8400-e29b-41d4-a716-446655440734', '550e8400-e29b-41d4-a716-446655440402', 'StartUp Ventures - Service revenue', 0.00, 7200.00),

-- June sales
('550e8400-e29b-41d4-a716-446655440760', '550e8400-e29b-41d4-a716-446655440735', '550e8400-e29b-41d4-a716-446655440102', 'ABC Manufacturing - Product sales', 18000.00, 0.00),
('550e8400-e29b-41d4-a716-446655440761', '550e8400-e29b-41d4-a716-446655440735', '550e8400-e29b-41d4-a716-446655440401', 'ABC Manufacturing - Product revenue', 0.00, 18000.00),

-- July sales
('550e8400-e29b-41d4-a716-446655440762', '550e8400-e29b-41d4-a716-446655440736', '550e8400-e29b-41d4-a716-446655440102', 'Global Tech - Consulting revenue', 22000.00, 0.00),
('550e8400-e29b-41d4-a716-446655440763', '550e8400-e29b-41d4-a716-446655440736', '550e8400-e29b-41d4-a716-446655440403', 'Global Tech - Consulting revenue', 0.00, 22000.00),

-- August sales
('550e8400-e29b-41d4-a716-446655440764', '550e8400-e29b-41d4-a716-446655440737', '550e8400-e29b-41d4-a716-446655440102', 'XYZ Services - Service revenue', 11500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440765', '550e8400-e29b-41d4-a716-446655440737', '550e8400-e29b-41d4-a716-446655440402', 'XYZ Services - Service revenue', 0.00, 11500.00),

-- September sales
('550e8400-e29b-41d4-a716-446655440766', '550e8400-e29b-41d4-a716-446655440738', '550e8400-e29b-41d4-a716-446655440102', 'Retail Solutions - Product sales', 16500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440767', '550e8400-e29b-41d4-a716-446655440738', '550e8400-e29b-41d4-a716-446655440401', 'Retail Solutions - Product revenue', 0.00, 16500.00),

-- October sales
('550e8400-e29b-41d4-a716-446655440768', '550e8400-e29b-41d4-a716-446655440739', '550e8400-e29b-41d4-a716-446655440102', 'StartUp Ventures - Project completion', 28500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440769', '550e8400-e29b-41d4-a716-446655440739', '550e8400-e29b-41d4-a716-446655440403', 'StartUp Ventures - Consulting revenue', 0.00, 28500.00),

-- November sales
('550e8400-e29b-41d4-a716-446655440770', '550e8400-e29b-41d4-a716-446655440740', '550e8400-e29b-41d4-a716-446655440102', 'ABC Manufacturing - Bulk order', 35000.00, 0.00),
('550e8400-e29b-41d4-a716-446655440771', '550e8400-e29b-41d4-a716-446655440740', '550e8400-e29b-41d4-a716-446655440401', 'ABC Manufacturing - Product revenue', 0.00, 35000.00),

-- December sales
('550e8400-e29b-41d4-a716-446655440772', '550e8400-e29b-41d4-a716-446655440741', '550e8400-e29b-41d4-a716-446655440102', 'Global Tech - Year-end project', 42000.00, 0.00),
('550e8400-e29b-41d4-a716-446655440773', '550e8400-e29b-41d4-a716-446655440741', '550e8400-e29b-41d4-a716-446655440403', 'Global Tech - Consulting revenue', 0.00, 42000.00);

-- Cash receipts from customers (partial payments to create aging scenarios)
INSERT INTO transactions (id, transaction_date, description, reference_number, contact_id, company_id, journal_type, status, created_by) VALUES
('550e8400-e29b-41d4-a716-446655440780', '2024-02-15', 'Payment from ABC Manufacturing', 'PAY-1001', '550e8400-e29b-41d4-a716-446655440600', 'Cash Receipts', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440781', '2024-03-20', 'Payment from XYZ Services', 'PAY-1002', '550e8400-e29b-41d4-a716-446655440601', 'Cash Receipts', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440782', '2024-05-10', 'Partial payment from Global Tech', 'PAY-1003', '550e8400-e29b-41d4-a716-446655440602', 'Cash Receipts', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440783', '2024-07-15', 'Payment from Retail Solutions', 'PAY-1004', '550e8400-e29b-41d4-a716-446655440603', 'Cash Receipts', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440784', '2024-08-20', 'Partial payment from StartUp Ventures', 'PAY-1005', '550e8400-e29b-41d4-a716-446655440604', 'Cash Receipts', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440785', '2024-09-25', 'Payment from ABC Manufacturing', 'PAY-1006', '550e8400-e29b-41d4-a716-446655440600', 'Cash Receipts', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440786', '2024-11-30', 'Partial payment from Global Tech', 'PAY-1007', '550e8400-e29b-41d4-a716-446655440602', 'Cash Receipts', 'posted', '550e8400-e29b-41d4-a716-446655440010');

-- Cash receipt line items
INSERT INTO transaction_line_items (id, transaction_id, account_id, description, debit_amount, credit_amount) VALUES
('550e8400-e29b-41d4-a716-446655440790', '550e8400-e29b-41d4-a716-446655440780', '550e8400-e29b-41d4-a716-446655440101', 'ABC Manufacturing payment', 8000.00, 0.00),
('550e8400-e29b-41d4-a716-446655440791', '550e8400-e29b-41d4-a716-446655440780', '550e8400-e29b-41d4-a716-446655440102', 'ABC Manufacturing invoice payment', 0.00, 8000.00),

('550e8400-e29b-41d4-a716-446655440792', '550e8400-e29b-41d4-a716-446655440781', '550e8400-e29b-41d4-a716-446655440101', 'XYZ Services payment', 8500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440793', '550e8400-e29b-41d4-a716-446655440781', '550e8400-e29b-41d4-a716-446655440102', 'XYZ Services invoice payment', 0.00, 8500.00),

('550e8400-e29b-41d4-a716-446655440794', '550e8400-e29b-41d4-a716-446655440782', '550e8400-e29b-41d4-a716-446655440101', 'Global Tech partial payment', 10000.00, 0.00),
('550e8400-e29b-41d4-a716-446655440795', '550e8400-e29b-41d4-a716-446655440782', '550e8400-e29b-41d4-a716-446655440102', 'Global Tech invoice payment', 0.00, 10000.00),

('550e8400-e29b-41d4-a716-446655440796', '550e8400-e29b-41d4-a716-446655440783', '550e8400-e29b-41d4-a716-446655440101', 'Retail Solutions payment', 9800.00, 0.00),
('550e8400-e29b-41d4-a716-446655440797', '550e8400-e29b-41d4-a716-446655440783', '550e8400-e29b-41d4-a716-446655440102', 'Retail Solutions invoice payment', 0.00, 9800.00),

('550e8400-e29b-41d4-a716-446655440798', '550e8400-e29b-41d4-a716-446655440784', '550e8400-e29b-41d4-a716-446655440101', 'StartUp Ventures partial payment', 4000.00, 0.00),
('550e8400-e29b-41d4-a716-446655440799', '550e8400-e29b-41d4-a716-446655440784', '550e8400-e29b-41d4-a716-446655440102', 'StartUp Ventures invoice payment', 0.00, 4000.00),

('550e8400-e29b-41d4-a716-446655440800', '550e8400-e29b-41d4-a716-446655440785', '550e8400-e29b-41d4-a716-446655440101', 'ABC Manufacturing payment', 18000.00, 0.00),
('550e8400-e29b-41d4-a716-446655440801', '550e8400-e29b-41d4-a716-446655440785', '550e8400-e29b-41d4-a716-446655440102', 'ABC Manufacturing invoice payment', 0.00, 18000.00),

('550e8400-e29b-41d4-a716-446655440802', '550e8400-e29b-41d4-a716-446655440786', '550e8400-e29b-41d4-a716-446655440101', 'Global Tech partial payment', 25000.00, 0.00),
('550e8400-e29b-41d4-a716-446655440803', '550e8400-e29b-41d4-a716-446655440786', '550e8400-e29b-41d4-a716-446655440102', 'Global Tech invoice payment', 0.00, 25000.00);

-- Operating expenses throughout the year
INSERT INTO transactions (id, transaction_date, description, reference_number, contact_id, company_id, journal_type, status, created_by) VALUES
('550e8400-e29b-41d4-a716-446655440810', '2024-02-01', 'February office rent', 'RENT-2024-02', '550e8400-e29b-41d4-a716-446655440610', 'General', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440811', '2024-03-01', 'March office rent', 'RENT-2024-03', '550e8400-e29b-41d4-a716-446655440610', 'General', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440812', '2024-04-01', 'April office rent', 'RENT-2024-04', '550e8400-e29b-41d4-a716-446655440610', 'General', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440813', '2024-05-01', 'May office rent', 'RENT-2024-05', '550e8400-e29b-41d4-a716-446655440610', 'General', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440814', '2024-06-01', 'June office rent', 'RENT-2024-06', '550e8400-e29b-41d4-a716-446655440610', 'General', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440815', '2024-07-01', 'July office rent', 'RENT-2024-07', '550e8400-e29b-41d4-a716-446655440610', 'General', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440816', '2024-08-01', 'August office rent', 'RENT-2024-08', '550e8400-e29b-41d4-a716-446655440610', 'General', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440817', '2024-09-01', 'September office rent', 'RENT-2024-09', '550e8400-e29b-41d4-a716-446655440610', 'General', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440818', '2024-10-01', 'October office rent', 'RENT-2024-10', '550e8400-e29b-41d4-a716-446655440610', 'General', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440819', '2024-11-01', 'November office rent', 'RENT-2024-11', '550e8400-e29b-41d4-a716-446655440610', 'General', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440820', '2024-12-01', 'December office rent', 'RENT-2024-12', '550e8400-e29b-41d4-a716-446655440610', 'General', 'posted', '550e8400-e29b-41d4-a716-446655440010'),

('550e8400-e29b-41d4-a716-446655440821', '2024-02-10', 'Office supplies purchase', 'SUP-2024-02', '550e8400-e29b-41d4-a716-446655440610', 'Purchases', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440822', '2024-05-15', 'Software licenses', 'SOFT-2024-05', '550e8400-e29b-41d4-a716-446655440611', 'Purchases', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440823', '2024-08-20', 'Marketing campaign', 'MKT-2024-08', '550e8400-e29b-41d4-a716-446655440610', 'General', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440824', '2024-11-25', 'Professional services', 'PRO-2024-11', '550e8400-e29b-41d4-a716-446655440613', 'General', 'posted', '550e8400-e29b-41d4-a716-446655440010');

-- Monthly rent expense line items
INSERT INTO transaction_line_items (id, transaction_id, account_id, description, debit_amount, credit_amount) VALUES
('550e8400-e29b-41d4-a716-446655440830', '550e8400-e29b-41d4-a716-446655440810', '550e8400-e29b-41d4-a716-446655440504', 'February office rent', 2500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440831', '550e8400-e29b-41d4-a716-446655440810', '550e8400-e29b-41d4-a716-446655440101', 'February rent payment', 0.00, 2500.00),

('550e8400-e29b-41d4-a716-446655440832', '550e8400-e29b-41d4-a716-446655440811', '550e8400-e29b-41d4-a716-446655440504', 'March office rent', 2500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440833', '550e8400-e29b-41d4-a716-446655440811', '550e8400-e29b-41d4-a716-446655440101', 'March rent payment', 0.00, 2500.00),

('550e8400-e29b-41d4-a716-446655440834', '550e8400-e29b-41d4-a716-446655440812', '550e8400-e29b-41d4-a716-446655440504', 'April office rent', 2500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440835', '550e8400-e29b-41d4-a716-446655440812', '550e8400-e29b-41d4-a716-446655440101', 'April rent payment', 0.00, 2500.00),

('550e8400-e29b-41d4-a716-446655440836', '550e8400-e29b-41d4-a716-446655440813', '550e8400-e29b-41d4-a716-446655440504', 'May office rent', 2500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440837', '550e8400-e29b-41d4-a716-446655440813', '550e8400-e29b-41d4-a716-446655440101', 'May rent payment', 0.00, 2500.00),

('550e8400-e29b-41d4-a716-446655440838', '550e8400-e29b-41d4-a716-446655440814', '550e8400-e29b-41d4-a716-446655440504', 'June office rent', 2500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440839', '550e8400-e29b-41d4-a716-446655440814', '550e8400-e29b-41d4-a716-446655440101', 'June rent payment', 0.00, 2500.00),

('550e8400-e29b-41d4-a716-446655440840', '550e8400-e29b-41d4-a716-446655440815', '550e8400-e29b-41d4-a716-446655440504', 'July office rent', 2500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440841', '550e8400-e29b-41d4-a716-446655440815', '550e8400-e29b-41d4-a716-446655440101', 'July rent payment', 0.00, 2500.00),

('550e8400-e29b-41d4-a716-446655440842', '550e8400-e29b-41d4-a716-446655440816', '550e8400-e29b-41d4-a716-446655440504', 'August office rent', 2500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440843', '550e8400-e29b-41d4-a716-446655440816', '550e8400-e29b-41d4-a716-446655440101', 'August rent payment', 0.00, 2500.00),

('550e8400-e29b-41d4-a716-446655440844', '550e8400-e29b-41d4-a716-446655440817', '550e8400-e29b-41d4-a716-446655440504', 'September office rent', 2500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440845', '550e8400-e29b-41d4-a716-446655440817', '550e8400-e29b-41d4-a716-446655440101', 'September rent payment', 0.00, 2500.00),

('550e8400-e29b-41d4-a716-446655440846', '550e8400-e29b-41d4-a716-446655440818', '550e8400-e29b-41d4-a716-446655440504', 'October office rent', 2500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440847', '550e8400-e29b-41d4-a716-446655440818', '550e8400-e29b-41d4-a716-446655440101', 'October rent payment', 0.00, 2500.00),

('550e8400-e29b-41d4-a716-446655440848', '550e8400-e29b-41d4-a716-446655440819', '550e8400-e29b-41d4-a716-446655440504', 'November office rent', 2500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440849', '550e8400-e29b-41d4-a716-446655440819', '550e8400-e29b-41d4-a716-446655440101', 'November rent payment', 0.00, 2500.00),

('550e8400-e29b-41d4-a716-446655440850', '550e8400-e29b-41d4-a716-446655440820', '550e8400-e29b-41d4-a716-446655440504', 'December office rent', 2500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440851', '550e8400-e29b-41d4-a716-446655440820', '550e8400-e29b-41d4-a716-446655440101', 'December rent payment', 0.00, 2500.00),

-- Other operating expenses
('550e8400-e29b-41d4-a716-446655440852', '550e8400-e29b-41d4-a716-446655440821', '550e8400-e29b-41d4-a716-446655440506', 'Office supplies purchase', 450.00, 0.00),
('550e8400-e29b-41d4-a716-446655440853', '550e8400-e29b-41d4-a716-446655440821', '550e8400-e29b-41d4-a716-446655440101', 'Office supplies payment', 0.00, 450.00),

('550e8400-e29b-41d4-a716-446655440854', '550e8400-e29b-41d4-a716-446655440822', '550e8400-e29b-41d4-a716-446655440511', 'Software licenses', 1200.00, 0.00),
('550e8400-e29b-41d4-a716-446655440855', '550e8400-e29b-41d4-a716-446655440822', '550e8400-e29b-41d4-a716-446655440101', 'Software license payment', 0.00, 1200.00),

('550e8400-e29b-41d4-a716-446655440856', '550e8400-e29b-41d4-a716-446655440823', '550e8400-e29b-41d4-a716-446655440507', 'Marketing campaign', 3500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440857', '550e8400-e29b-41d4-a716-446655440823', '550e8400-e29b-41d4-a716-446655440101', 'Marketing campaign payment', 0.00, 3500.00),

('550e8400-e29b-41d4-a716-446655440858', '550e8400-e29b-41d4-a716-446655440824', '550e8400-e29b-41d4-a716-446655440509', 'Professional services', 2800.00, 0.00),
('550e8400-e29b-41d4-a716-446655440859', '550e8400-e29b-41d4-a716-446655440824', '550e8400-e29b-41d4-a716-446655440101', 'Professional services payment', 0.00, 2800.00);

-- Year-end adjusting entries
INSERT INTO transactions (id, transaction_date, description, reference_number, company_id, journal_type, status, created_by) VALUES
('550e8400-e29b-41d4-a716-446655440870', '2024-12-31', 'Depreciation expense - Office Equipment', 'DEPR-2024', '550e8400-e29b-41d4-a716-446655440001', 'General', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440871', '2024-12-31', 'Depreciation expense - Computer Equipment', 'DEPR-2024-PC', '550e8400-e29b-41d4-a716-446655440001', 'General', 'posted', '550e8400-e29b-41d4-a716-446655440010'),
('550e8400-e29b-41d4-a716-446655440872', '2024-12-31', 'Bank service charges', 'BANK-2024', '550e8400-e29b-41d4-a716-446655440001', 'General', 'posted', '550e8400-e29b-41d4-a716-446655440010');

-- Year-end adjusting entry line items
INSERT INTO transaction_line_items (id, transaction_id, account_id, description, debit_amount, credit_amount) VALUES
('550e8400-e29b-41d4-a716-446655440880', '550e8400-e29b-41d4-a716-446655440870', '550e8400-e29b-41d4-a716-446655440511', 'Office equipment depreciation', 1500.00, 0.00),
('550e8400-e29b-41d4-a716-446655440881', '550e8400-e29b-41d4-a716-446655440870', '550e8400-e29b-41d4-a716-446655440106', 'Accumulated depreciation - Office Equipment', 0.00, 1500.00),

('550e8400-e29b-41d4-a716-446655440882', '550e8400-e29b-41d4-a716-446655440871', '550e8400-e29b-41d4-a716-446655440511', 'Computer equipment depreciation', 3000.00, 0.00),
('550e8400-e29b-41d4-a716-446655440883', '550e8400-e29b-41d4-a716-446655440871', '550e8400-e29b-41d4-a716-446655440108', 'Accumulated depreciation - Computer Equipment', 0.00, 3000.00),

('550e8400-e29b-41d4-a716-446655440884', '550e8400-e29b-41d4-a716-446655440872', '550e8400-e29b-41d4-a716-446655440513', 'Bank service charges', 150.00, 0.00),
('550e8400-e29b-41d4-a716-446655440885', '550e8400-e29b-41d4-a716-446655440872', '550e8400-e29b-41d4-a716-446655440101', 'Bank service charges', 0.00, 150.00);

-- Create some mock invoice data for AR aging testing (since we don't have the invoices table in the current schema)
-- Note: This would need to be adapted when the invoices/payments tables are fully implemented
-- For now, the AR aging report will work with transaction_line_items as a proxy

COMMIT;