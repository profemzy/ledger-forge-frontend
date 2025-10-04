use axum::http::{header, HeaderName, HeaderValue};
use axum_test::TestServer;
use serde_json::{json, Value};
use serial_test::serial;

mod common;
use common::{setup_test_db, cleanup_test_db, TestUser, TEST_JWT_SECRET};
use common::{assert_success_response, assert_error_response, assert_valid_uuid, assert_valid_jwt};

use ledger_forge::services::{AuthService, AccountService, TransactionService};
use ledger_forge::routes::create_routes;

async fn create_test_server() -> TestServer {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let auth_service = AuthService::new(TEST_JWT_SECRET.to_string());
    let account_service = AccountService::new();
    let transaction_service = TransactionService::new();
    let app = create_routes(pool, auth_service, account_service, transaction_service);

    TestServer::new(app).unwrap()
}

#[tokio::test]
#[serial]
async fn test_health_check() {
    let server = create_test_server().await;

    let response = server.get("/api/v1/health").await;

    response.assert_status_ok();

    let json: Value = response.json();
    assert_eq!(json["status"], "ok");
    assert_eq!(json["database"], "healthy");
}

#[tokio::test]
#[serial]
async fn test_register_valid_user_returns_201() {
    let server = create_test_server().await;
    let user = TestUser::admin();

    let response = server
        .post("/api/v1/auth/register")
        .json(&json!({
            "username": user.username,
            "email": user.email,
            "password": user.password,
            "role": user.role
        }))
        .await;

    response.assert_status(axum::http::StatusCode::CREATED);

    let json: Value = response.json();
    assert_success_response(&json);

    let data = &json["data"];
    assert_valid_jwt(data["access_token"].as_str().unwrap());
    assert_valid_jwt(data["refresh_token"].as_str().unwrap());

    let user_data = &data["user"];
    assert_eq!(user_data["username"], user.username);
    assert_eq!(user_data["email"], user.email);
    assert_eq!(user_data["role"], user.role);
    assert_valid_uuid(user_data["id"].as_str().unwrap());
    assert!(user_data.get("password_hash").is_none()); // Password should not be returned
}

#[tokio::test]
#[serial]
async fn test_register_duplicate_username_returns_409() {
    let server = create_test_server().await;
    let user = TestUser::with_username("duplicate");

    // Register first time
    server
        .post("/api/v1/auth/register")
        .json(&json!({
            "username": user.username,
            "email": user.email,
            "password": user.password,
            "role": user.role
        }))
        .await
        .assert_status(axum::http::StatusCode::CREATED);

    // Register second time with same username
    let response = server
        .post("/api/v1/auth/register")
        .json(&json!({
            "username": user.username,
            "email": "different@test.com",
            "password": user.password,
            "role": user.role
        }))
        .await;

    response.assert_status(axum::http::StatusCode::CONFLICT);

    let json: Value = response.json();
    assert_error_response(&json);
}

#[tokio::test]
#[serial]
async fn test_register_duplicate_email_returns_409() {
    let server = create_test_server().await;
    let user = TestUser::with_username("user1");

    // Register first time
    server
        .post("/api/v1/auth/register")
        .json(&json!({
            "username": user.username,
            "email": user.email,
            "password": user.password,
            "role": user.role
        }))
        .await
        .assert_status(axum::http::StatusCode::CREATED);

    // Register second time with same email
    let response = server
        .post("/api/v1/auth/register")
        .json(&json!({
            "username": "different_user",
            "email": user.email,
            "password": user.password,
            "role": user.role
        }))
        .await;

    response.assert_status(axum::http::StatusCode::CONFLICT);
}

#[tokio::test]
#[serial]
async fn test_register_invalid_email_returns_400() {
    let server = create_test_server().await;

    let response = server
        .post("/api/v1/auth/register")
        .json(&json!({
            "username": "testuser",
            "email": "not-an-email",
            "password": "TestPass123!",
            "role": "viewer"
        }))
        .await;

    response.assert_status(axum::http::StatusCode::BAD_REQUEST);

    let json: Value = response.json();
    assert_error_response(&json);
}

#[tokio::test]
#[serial]
async fn test_login_valid_credentials_returns_200() {
    let server = create_test_server().await;
    let user = TestUser::regular();

    // First register
    server
        .post("/api/v1/auth/register")
        .json(&json!({
            "username": user.username,
            "email": user.email,
            "password": user.password,
            "role": user.role
        }))
        .await;

    // Then login
    let response = server
        .post("/api/v1/auth/login")
        .json(&json!({
            "username": user.username,
            "password": user.password
        }))
        .await;

    response.assert_status_ok();

    let json: Value = response.json();
    assert_success_response(&json);

    let data = &json["data"];
    assert_valid_jwt(data["access_token"].as_str().unwrap());
    assert_valid_jwt(data["refresh_token"].as_str().unwrap());

    let user_data = &data["user"];
    assert_eq!(user_data["username"], user.username);
    assert_eq!(user_data["email"], user.email);
}

#[tokio::test]
#[serial]
async fn test_login_invalid_username_returns_401() {
    let server = create_test_server().await;

    let response = server
        .post("/api/v1/auth/login")
        .json(&json!({
            "username": "nonexistent",
            "password": "TestPass123!"
        }))
        .await;

    response.assert_status(axum::http::StatusCode::UNAUTHORIZED);

    let json: Value = response.json();
    assert_error_response(&json);
}

#[tokio::test]
#[serial]
async fn test_login_invalid_password_returns_401() {
    let server = create_test_server().await;
    let user = TestUser::with_username("testuser2");

    // Register user
    server
        .post("/api/v1/auth/register")
        .json(&json!({
            "username": user.username,
            "email": user.email,
            "password": user.password,
            "role": user.role
        }))
        .await;

    // Login with wrong password
    let response = server
        .post("/api/v1/auth/login")
        .json(&json!({
            "username": user.username,
            "password": "WrongPassword123!"
        }))
        .await;

    response.assert_status(axum::http::StatusCode::UNAUTHORIZED);

    let json: Value = response.json();
    assert_error_response(&json);
}

#[tokio::test]
#[serial]
async fn test_get_me_with_valid_token_returns_200() {
    let server = create_test_server().await;
    let user = TestUser::with_username("metest");

    // Register and get token
    let register_response = server
        .post("/api/v1/auth/register")
        .json(&json!({
            "username": user.username,
            "email": user.email,
            "password": user.password,
            "role": user.role
        }))
        .await;

    let register_json: Value = register_response.json();
    let access_token = register_json["data"]["access_token"].as_str().unwrap();

    // Get current user
    let response = server
        .get("/api/v1/auth/me")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap()
        )
        .await;

    response.assert_status_ok();

    let json: Value = response.json();
    assert_success_response(&json);

    let data = &json["data"];
    assert_eq!(data["username"], user.username);
    assert_eq!(data["email"], user.email);
}

#[tokio::test]
#[serial]
async fn test_get_me_without_token_returns_401() {
    let server = create_test_server().await;

    let response = server
        .get("/api/v1/auth/me")
        .await;

    response.assert_status(axum::http::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[serial]
async fn test_get_me_with_invalid_token_returns_401() {
    let server = create_test_server().await;

    let response = server
        .get("/api/v1/auth/me")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_static("Bearer invalid.token.here")
        )
        .await;

    response.assert_status(axum::http::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[serial]
async fn test_refresh_token_valid_returns_200() {
    let server = create_test_server().await;
    let user = TestUser::with_username("refreshtest");

    // Register and get refresh token
    let register_response = server
        .post("/api/v1/auth/register")
        .json(&json!({
            "username": user.username,
            "email": user.email,
            "password": user.password,
            "role": user.role
        }))
        .await;

    let register_json: Value = register_response.json();
    let refresh_token = register_json["data"]["refresh_token"].as_str().unwrap();

    // Refresh token
    let response = server
        .post("/api/v1/auth/refresh")
        .json(&json!({
            "refresh_token": refresh_token
        }))
        .await;

    response.assert_status_ok();

    let json: Value = response.json();
    assert_success_response(&json);

    let data = &json["data"];
    assert_valid_jwt(data["access_token"].as_str().unwrap());
}

#[tokio::test]
#[serial]
async fn test_refresh_token_invalid_returns_401() {
    let server = create_test_server().await;

    let response = server
        .post("/api/v1/auth/refresh")
        .json(&json!({
            "refresh_token": "invalid.refresh.token"
        }))
        .await;

    response.assert_status(axum::http::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[serial]
async fn test_complete_auth_workflow() {
    let server = create_test_server().await;
    let user = TestUser::with_username("workflow");

    // Step 1: Register
    let register_response = server
        .post("/api/v1/auth/register")
        .json(&json!({
            "username": user.username,
            "email": user.email,
            "password": user.password,
            "role": user.role
        }))
        .await;
    register_response.assert_status(axum::http::StatusCode::CREATED);

    // Step 2: Login
    let login_response = server
        .post("/api/v1/auth/login")
        .json(&json!({
            "username": user.username,
            "password": user.password
        }))
        .await;
    login_response.assert_status_ok();

    let login_json: Value = login_response.json();
    let access_token = login_json["data"]["access_token"].as_str().unwrap();

    // Step 3: Access protected route
    let me_response = server
        .get("/api/v1/auth/me")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap()
        )
        .await;
    me_response.assert_status_ok();

    let me_json: Value = me_response.json();
    assert_eq!(me_json["data"]["username"], user.username);
}
