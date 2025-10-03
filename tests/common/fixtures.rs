use fake::faker::internet::en::*;
use fake::faker::name::en::*;
use fake::Fake;

/// Generate test user data
pub struct TestUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

impl TestUser {
    pub fn admin() -> Self {
        Self {
            username: "testadmin".to_string(),
            email: "admin@test.com".to_string(),
            password: "TestPass123!".to_string(),
            role: "admin".to_string(),
        }
    }

    pub fn regular() -> Self {
        Self {
            username: "testuser".to_string(),
            email: "user@test.com".to_string(),
            password: "TestPass123!".to_string(),
            role: "viewer".to_string(),
        }
    }

    pub fn random() -> Self {
        Self {
            username: Username().fake::<String>(),
            email: SafeEmail().fake::<String>(),
            password: "TestPass123!".to_string(),
            role: "viewer".to_string(),
        }
    }

    pub fn with_username(username: &str) -> Self {
        Self {
            username: username.to_string(),
            email: format!("{}@test.com", username),
            password: "TestPass123!".to_string(),
            role: "viewer".to_string(),
        }
    }
}

/// JWT test secret - matches what's used in tests
pub const TEST_JWT_SECRET: &str = "test-secret-key-for-testing-only";

/// Common test passwords
pub mod passwords {
    pub const VALID: &str = "TestPass123!";
    pub const WEAK: &str = "weak";
    pub const EMPTY: &str = "";
    pub const LONG: &str = "ThisIsAVeryLongPasswordThatExceedsNormalLimitsAndShouldBeTestedForEdgeCases123!@#";
}

/// Common test usernames
pub mod usernames {
    pub const VALID: &str = "testuser";
    pub const SHORT: &str = "ab";
    pub const LONG: &str = "thisusernameiswaytoolongandexceedstypicallimits";
    pub const SPECIAL_CHARS: &str = "test@user#123";
    pub const EMPTY: &str = "";
}

/// Common test emails
pub mod emails {
    pub const VALID: &str = "test@example.com";
    pub const INVALID: &str = "not-an-email";
    pub const EMPTY: &str = "";
}
