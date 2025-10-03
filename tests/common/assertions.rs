use serde_json::Value;

/// Assert that a JSON response contains a specific field
pub fn assert_json_field_exists(json: &Value, field: &str) {
    assert!(
        json.get(field).is_some(),
        "Expected field '{}' not found in JSON: {:?}",
        field,
        json
    );
}

/// Assert that a JSON response has a specific field with a value
pub fn assert_json_field_eq(json: &Value, field: &str, expected: &str) {
    let value = json.get(field).expect(&format!("Field '{}' not found", field));
    let value_str = value.as_str().expect(&format!("Field '{}' is not a string", field));
    assert_eq!(
        value_str, expected,
        "Expected field '{}' to be '{}', got '{}'",
        field, expected, value_str
    );
}

/// Assert that JSON has success=true
pub fn assert_success_response(json: &Value) {
    assert_json_field_exists(json, "success");
    assert!(
        json["success"].as_bool().unwrap_or(false),
        "Expected success=true, got: {:?}",
        json
    );
}

/// Assert that JSON has an error field
pub fn assert_error_response(json: &Value) {
    assert_json_field_exists(json, "error");
}

/// Assert that a string is a valid UUID
pub fn assert_valid_uuid(s: &str) {
    uuid::Uuid::parse_str(s).expect(&format!("'{}' is not a valid UUID", s));
}

/// Assert that a string is a valid JWT token
pub fn assert_valid_jwt(token: &str) {
    assert!(
        token.split('.').count() == 3,
        "Invalid JWT format. Expected 3 parts, got: {}",
        token.split('.').count()
    );
}
