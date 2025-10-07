use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

use crate::services::{AccountService, CacheService};
use crate::utils::{
    parse_accounts_csv, validate_account_record, AccountCsvRecord,
    ImportResult, ImportError, AppError, Result
};

#[derive(Clone)]
pub struct ImportService {
    account_service: AccountService,
}

impl ImportService {
    pub fn new(cache_service: CacheService) -> Self {
        Self {
            account_service: AccountService::new_with_cache(cache_service),
        }
    }

    /// Import Chart of Accounts from CSV data
    pub async fn import_accounts_from_csv(
        &self,
        pool: &PgPool,
        csv_data: &[u8],
    ) -> Result<ImportResult> {
        // Parse CSV
        let records = parse_accounts_csv(csv_data)?;
        let total_rows = records.len();

        let mut successful = 0;
        let mut failed = 0;
        let mut errors = Vec::new();
        let mut parent_account_map: HashMap<String, Uuid> = HashMap::new();

        // First pass: Create accounts without parents
        for (index, record) in records.iter().enumerate() {
            let row_number = index + 2; // +2 because row 1 is header, and we're 0-indexed

            // Skip records with parent codes in first pass
            if record.parent_code.is_some() && !record.parent_code.as_ref().unwrap().trim().is_empty() {
                continue;
            }

            match self.create_account_from_record(pool, record, &parent_account_map, row_number).await {
                Ok(account_id) => {
                    parent_account_map.insert(record.code.clone(), account_id);
                    successful += 1;
                }
                Err(e) => {
                    failed += 1;
                    errors.push(ImportError {
                        row_number,
                        code: record.code.clone(),
                        error_message: e.to_string(),
                    });
                }
            }
        }

        // Second pass: Create accounts with parents
        for (index, record) in records.iter().enumerate() {
            let row_number = index + 2;

            // Only process records with parent codes
            if record.parent_code.is_none() || record.parent_code.as_ref().unwrap().trim().is_empty() {
                continue;
            }

            match self.create_account_from_record(pool, record, &parent_account_map, row_number).await {
                Ok(account_id) => {
                    parent_account_map.insert(record.code.clone(), account_id);
                    successful += 1;
                }
                Err(e) => {
                    failed += 1;
                    errors.push(ImportError {
                        row_number,
                        code: record.code.clone(),
                        error_message: e.to_string(),
                    });
                }
            }
        }

        Ok(ImportResult {
            total_rows,
            successful,
            failed,
            errors,
        })
    }

    /// Helper function to create an account from a CSV record
    async fn create_account_from_record(
        &self,
        pool: &PgPool,
        record: &AccountCsvRecord,
        parent_account_map: &HashMap<String, Uuid>,
        row_number: usize,
    ) -> Result<Uuid> {
        // Validate and convert record
        let account_request = validate_account_record(record, parent_account_map)
            .map_err(|e| AppError::ValidationError(
                format!("Row {}: {}", row_number, e)
            ))?;

        // Check if parent exists if parent_code is provided
        if let Some(ref parent_code) = record.parent_code {
            if !parent_code.trim().is_empty() && !parent_account_map.contains_key(parent_code) {
                return Err(AppError::ValidationError(
                    format!("Row {}: Parent account with code '{}' not found", row_number, parent_code)
                ));
            }
        }

        // Create the account
        let account = self.account_service
            .create_account(pool, account_request)
            .await
            .map_err(|e| AppError::ValidationError(
                format!("Row {}: Failed to create account - {}", row_number, e)
            ))?;

        Ok(account.id)
    }

    /// Get CSV template for Chart of Accounts
    pub fn get_accounts_csv_template() -> String {
        crate::utils::csv_import::generate_accounts_csv_template()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_template() {
        let template = ImportService::get_accounts_csv_template();
        assert!(template.contains("code,name,account_type"));
        assert!(!template.is_empty());
    }
}
