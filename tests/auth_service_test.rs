use ledger_forge::services::auth::AuthService;
use ledger_forge::models::{User, UserRole};
use uuid::Uuid;
use chrono::Utc;

mod common;

const TEST_SECRET: &str = "test-secret-key";

fn create_test_auth_service() -> AuthService {
    AuthService::new(TEST_SECRET.to_string())
}

fn create_test_user() -> User {
    User {
        id: Uuid::new_v4(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password_hash: "fake_hash".to_string(),
        role: UserRole::Viewer,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[cfg(test)]
mod password_tests {
    use super::*;

    #[test]
    fn test_hash_password_success() {
        let auth_service = create_test_auth_service();
        let password = "SecurePass123!";

        let result = auth_service.hash_password(password);

        assert!(result.is_ok());
        let hash = result.unwrap();
        assert!(!hash.is_empty());
        assert_ne!(hash, password); // Hash should be different from password
        assert!(hash.starts_with("$argon2")); // Argon2 hash format
    }

    #[test]
    fn test_hash_password_produces_different_hashes() {
        let auth_service = create_test_auth_service();
        let password = "SecurePass123!";

        let hash1 = auth_service.hash_password(password).unwrap();
        let hash2 = auth_service.hash_password(password).unwrap();

        // Same password should produce different hashes due to different salts
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_password_correct_password() {
        let auth_service = create_test_auth_service();
        let password = "SecurePass123!";

        let hash = auth_service.hash_password(password).unwrap();
        let result = auth_service.verify_password(password, &hash);

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_verify_password_incorrect_password() {
        let auth_service = create_test_auth_service();
        let correct_password = "SecurePass123!";
        let wrong_password = "WrongPass456!";

        let hash = auth_service.hash_password(correct_password).unwrap();
        let result = auth_service.verify_password(wrong_password, &hash);

        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_verify_password_invalid_hash() {
        let auth_service = create_test_auth_service();
        let password = "SecurePass123!";
        let invalid_hash = "not-a-valid-hash";

        let result = auth_service.verify_password(password, invalid_hash);

        assert!(result.is_err());
    }

    #[test]
    fn test_hash_empty_password() {
        let auth_service = create_test_auth_service();
        let password = "";

        let result = auth_service.hash_password(password);

        // Should succeed even with empty password
        assert!(result.is_ok());
    }

    #[test]
    fn test_hash_very_long_password() {
        let auth_service = create_test_auth_service();
        let password = "a".repeat(1000);

        let result = auth_service.hash_password(&password);

        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod jwt_tests {
    use super::*;
    use jsonwebtoken::{decode, DecodingKey, Validation};
    use ledger_forge::services::auth::Claims;

    #[test]
    fn test_generate_access_token_success() {
        let auth_service = create_test_auth_service();
        let user = create_test_user();

        let result = auth_service.generate_access_token(&user);

        assert!(result.is_ok());
        let token = result.unwrap();
        assert!(!token.is_empty());
        assert_eq!(token.split('.').count(), 3); // JWT has 3 parts
    }

    #[test]
    fn test_generate_refresh_token_success() {
        let auth_service = create_test_auth_service();
        let user = create_test_user();

        let result = auth_service.generate_refresh_token(&user);

        assert!(result.is_ok());
        let token = result.unwrap();
        assert!(!token.is_empty());
        assert_eq!(token.split('.').count(), 3);
    }

    #[test]
    fn test_access_and_refresh_tokens_are_different() {
        let auth_service = create_test_auth_service();
        let user = create_test_user();

        let access_token = auth_service.generate_access_token(&user).unwrap();
        let refresh_token = auth_service.generate_refresh_token(&user).unwrap();

        assert_ne!(access_token, refresh_token);
    }

    #[test]
    fn test_validate_token_success() {
        let auth_service = create_test_auth_service();
        let user = create_test_user();

        let token = auth_service.generate_access_token(&user).unwrap();
        let result = auth_service.validate_token(&token);

        assert!(result.is_ok());
        let token_data = result.unwrap();
        assert_eq!(token_data.claims.username, user.username);
        assert_eq!(token_data.claims.email, user.email);
        assert_eq!(token_data.claims.sub, user.id.to_string());
    }

    #[test]
    fn test_validate_token_with_wrong_secret() {
        let auth_service1 = create_test_auth_service();
        let auth_service2 = AuthService::new("different-secret".to_string());
        let user = create_test_user();

        let token = auth_service1.generate_access_token(&user).unwrap();
        let result = auth_service2.validate_token(&token);

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_token_malformed() {
        let auth_service = create_test_auth_service();
        let malformed_token = "not.a.valid.jwt";

        let result = auth_service.validate_token(malformed_token);

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_token_empty() {
        let auth_service = create_test_auth_service();

        let result = auth_service.validate_token("");

        assert!(result.is_err());
    }

    #[test]
    fn test_token_contains_correct_claims() {
        let auth_service = create_test_auth_service();
        let user = create_test_user();

        let token = auth_service.generate_access_token(&user).unwrap();

        // Manually decode to verify claims
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(TEST_SECRET.as_bytes()),
            &Validation::default(),
        ).unwrap();

        assert_eq!(token_data.claims.username, "testuser");
        assert_eq!(token_data.claims.email, "test@example.com");
        assert!(token_data.claims.exp > Utc::now().timestamp());
        assert!(token_data.claims.iat <= Utc::now().timestamp());
    }

    #[test]
    fn test_token_role_preserved() {
        let auth_service = create_test_auth_service();
        let mut user = create_test_user();
        user.role = UserRole::Admin;

        let token = auth_service.generate_access_token(&user).unwrap();
        let token_data = auth_service.validate_token(&token).unwrap();

        assert_eq!(token_data.claims.role, UserRole::Admin);
    }

    #[test]
    fn test_refresh_token_has_longer_expiry() {
        let auth_service = create_test_auth_service();
        let user = create_test_user();

        let access_token = auth_service.generate_access_token(&user).unwrap();
        let refresh_token = auth_service.generate_refresh_token(&user).unwrap();

        let access_data = decode::<Claims>(
            &access_token,
            &DecodingKey::from_secret(TEST_SECRET.as_bytes()),
            &Validation::default(),
        ).unwrap();

        let refresh_data = decode::<Claims>(
            &refresh_token,
            &DecodingKey::from_secret(TEST_SECRET.as_bytes()),
            &Validation::default(),
        ).unwrap();

        assert!(refresh_data.claims.exp > access_data.claims.exp);
    }
}

#[cfg(test)]
mod auth_service_creation_tests {
    use super::*;

    #[test]
    fn test_auth_service_creation() {
        let secret = "test-secret".to_string();
        let auth_service = AuthService::new(secret);

        // Service should be created successfully
        // We can't directly test private fields, but we can test it works
        let user = create_test_user();
        assert!(auth_service.generate_access_token(&user).is_ok());
    }

    #[test]
    fn test_auth_service_with_empty_secret() {
        let auth_service = AuthService::new("".to_string());
        let user = create_test_user();

        // Should still work (though not recommended for production)
        assert!(auth_service.generate_access_token(&user).is_ok());
    }
}
