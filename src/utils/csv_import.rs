use csv::ReaderBuilder;
use serde::Deserialize;
use std::io::Read;
use uuid::Uuid;

use crate::models::{AccountType, CreateAccountRequest};
use crate::utils::{AppError, Result};

/// CSV record for importing Chart of Accounts
#[derive(Debug, Deserialize)]
pub struct AccountCsvRecord {
    pub code: String,
    pub name: String,
    pub account_type: String,
    #[serde(default)]
    pub parent_code: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    pub description: Option<String>,
}

/// Result of CSV import operation
#[derive(Debug)]
pub struct ImportResult {
    pub total_rows: usize,
    pub successful: usize,
    pub failed: usize,
    pub errors: Vec<ImportError>,
}

/// Error details for failed import rows
#[derive(Debug)]
pub struct ImportError {
    pub row_number: usize,
    pub code: String,
    pub error_message: String,
}

/// Parse Chart of Accounts from CSV data
pub fn parse_accounts_csv<R: Read>(reader: R) -> Result<Vec<AccountCsvRecord>> {
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(reader);

    let mut records = Vec::new();
    let mut row_number = 1; // Start at 1 (header is row 0)

    for result in csv_reader.deserialize() {
        row_number += 1;
        match result {
            Ok(record) => records.push(record),
            Err(e) => {
                return Err(AppError::ValidationError(
                    format!("CSV parsing error at row {}: {}", row_number, e)
                ));
            }
        }
    }

    if records.is_empty() {
        return Err(AppError::ValidationError(
            "CSV file contains no data rows".to_string()
        ));
    }

    Ok(records)
}

/// Validate and convert CSV record to CreateAccountRequest
pub fn validate_account_record(
    record: &AccountCsvRecord,
    parent_account_map: &std::collections::HashMap<String, Uuid>,
) -> Result<CreateAccountRequest> {
    // Validate account code
    if record.code.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Account code cannot be empty".to_string()
        ));
    }

    // Validate account name
    if record.name.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Account name cannot be empty".to_string()
        ));
    }

    // Parse and validate account type
    let account_type = match record.account_type.trim().to_lowercase().as_str() {
        "asset" => AccountType::Asset,
        "liability" => AccountType::Liability,
        "equity" => AccountType::Equity,
        "revenue" | "income" => AccountType::Revenue,
        "expense" => AccountType::Expense,
        _ => {
            return Err(AppError::ValidationError(
                format!("Invalid account type '{}'. Must be one of: Asset, Liability, Equity, Revenue, Expense", record.account_type)
            ));
        }
    };

    // Resolve parent account ID if parent code is provided
    let parent_account_id = if let Some(ref parent_code) = record.parent_code {
        if !parent_code.trim().is_empty() {
            parent_account_map.get(parent_code).copied()
        } else {
            None
        }
    } else {
        None
    };

    Ok(CreateAccountRequest {
        code: record.code.trim().to_string(),
        name: record.name.trim().to_string(),
        account_type,
        parent_account_id,
        company_id: None,
    })
}

/// Generate sample CSV template for Chart of Accounts
pub fn generate_accounts_csv_template() -> String {
    let template = r#"code,name,account_type,parent_code,description
1000,Cash,Asset,,Primary cash account
1010,Checking Account,Asset,1000,Business checking account
1100,Accounts Receivable,Asset,,Customer receivables
2000,Accounts Payable,Liability,,Vendor payables
3000,Owner's Equity,Equity,,Owner's capital
4000,Sales Revenue,Revenue,,Product and service sales
5000,Cost of Goods Sold,Expense,,Direct costs
6000,Operating Expenses,Expense,,General operating expenses
6010,Rent Expense,Expense,6000,Office rent
6020,Utilities,Expense,6000,Electricity and water"#;

    template.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_valid_csv() {
        let csv_data = r#"code,name,account_type,parent_code,description
1000,Cash,Asset,,Cash account
2000,Accounts Payable,Liability,,Vendor payables"#;

        let records = parse_accounts_csv(csv_data.as_bytes()).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].code, "1000");
        assert_eq!(records[0].name, "Cash");
        assert_eq!(records[0].account_type, "Asset");
    }

    #[test]
    fn test_validate_account_record() {
        let record = AccountCsvRecord {
            code: "1000".to_string(),
            name: "Cash".to_string(),
            account_type: "Asset".to_string(),
            parent_code: None,
            description: None,
        };

        let parent_map = HashMap::new();
        let request = validate_account_record(&record, &parent_map).unwrap();

        assert_eq!(request.code, "1000");
        assert_eq!(request.name, "Cash");
        assert_eq!(request.account_type, AccountType::Asset);
    }

    #[test]
    fn test_validate_invalid_account_type() {
        let record = AccountCsvRecord {
            code: "1000".to_string(),
            name: "Cash".to_string(),
            account_type: "InvalidType".to_string(),
            parent_code: None,
            description: None,
        };

        let parent_map = HashMap::new();
        let result = validate_account_record(&record, &parent_map);

        assert!(result.is_err());
    }

    #[test]
    fn test_generate_template() {
        let template = generate_accounts_csv_template();
        assert!(template.contains("code,name,account_type"));
        assert!(template.contains("1000,Cash,Asset"));
    }
}
