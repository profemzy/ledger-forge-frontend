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
