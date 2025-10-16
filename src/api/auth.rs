use serde::{Deserialize, Serialize};

use crate::api::client;
use crate::state::User;
use crate::utils::storage;

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: User,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    pub access_token: String,
}

pub async fn login(username: String, password: String) -> Result<AuthResponse, String> {
    let body = LoginRequest { username, password };
    let res: AuthResponse = client::post("/auth/login", &body).await?;
    storage::set_token(&res.access_token);
    storage::set_refresh_token(&res.refresh_token);
    Ok(res)
}

pub async fn me() -> Result<User, String> {
    client::get::<User>("/auth/me").await
}

pub async fn refresh() -> Result<String, String> {
    let Some(refresh_token) = storage::get_refresh_token() else {
        return Err("No refresh token".into());
    };
    let body = serde_json::json!({"refresh_token": refresh_token});
    let res: TokenResponse = client::post("/auth/refresh", &body).await?;
    storage::set_token(&res.access_token);
    Ok(res.access_token)
}

pub fn logout() {
    storage::remove_token();
    storage::remove_refresh_token();
}
