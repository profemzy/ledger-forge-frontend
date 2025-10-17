use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::utils::storage;

#[derive(Debug, serde::Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
    pub message: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct TokenResponse {
    access_token: String,
}

fn api_base() -> String {
    // For now use localhost; later this could read from <meta> or env
    "http://localhost:3000/api/v1".to_string()
}

pub async fn get<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    request::<T>("GET", path, None).await
}

pub async fn post<B: Serialize, T: DeserializeOwned>(path: &str, body: &B) -> Result<T, String> {
    let json = serde_json::to_string(body).map_err(|e| e.to_string())?;
    request::<T>("POST", path, Some(json)).await
}

pub async fn put<B: Serialize, T: DeserializeOwned>(path: &str, body: &B) -> Result<T, String> {
    let json = serde_json::to_string(body).map_err(|e| e.to_string())?;
    request::<T>("PUT", path, Some(json)).await
}

pub async fn delete<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    request::<T>("DELETE", path, None).await
}

async fn request<T: DeserializeOwned>(method: &str, path: &str, body: Option<String>) -> Result<T, String> {
    let url = format!("{}{}", api_base(), path);

    let mut builder = match method {
        "GET" => Request::get(&url),
        "POST" => Request::post(&url),
        "PUT" => Request::put(&url),
        "DELETE" => Request::delete(&url),
        _ => return Err("Unsupported method".into()),
    };

    if let Some(token) = storage::get_token() {
        let header_val = format!("Bearer {}", token);
        builder = builder.header("Authorization", &header_val);
    }
    let resp = if let Some(json) = body.clone() {
        let req = builder
            .header("Content-Type", "application/json")
            .body(json)
            .map_err(|e| e.to_string())?;
        req.send().await.map_err(|e| e.to_string())?
    } else {
        builder.send().await.map_err(|e| e.to_string())?
    };

    if resp.status() == 401 {
        if let Some(refresh_token) = storage::get_refresh_token() {
            if let Ok(new_access) = do_refresh(&refresh_token).await {
                storage::set_token(&new_access);
                // retry once
                let mut retry_builder = match method {
                    "GET" => Request::get(&url),
                    "POST" => Request::post(&url),
                    "PUT" => Request::put(&url),
                    "DELETE" => Request::delete(&url),
                    _ => return Err("Unsupported method".into()),
                };
                let header_val = format!("Bearer {}", new_access);
                retry_builder = retry_builder.header("Authorization", &header_val);
                let resp = if let Some(json) = body {
                    let req = retry_builder
                        .header("Content-Type", "application/json")
                        .body(json)
                        .map_err(|e| e.to_string())?;
                    req.send().await.map_err(|e| e.to_string())?
                } else {
                    retry_builder.send().await.map_err(|e| e.to_string())?
                };
                return parse_response::<T>(resp).await;
            }
        }
        storage::remove_token();
        storage::remove_refresh_token();
        return Err("Unauthorized".to_string());
    }

    parse_response::<T>(resp).await
}

async fn parse_response<T: DeserializeOwned>(resp: gloo_net::http::Response) -> Result<T, String> {
    if !resp.ok() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("HTTP {}: {}", status, text));
    }
    let text = resp.text().await.map_err(|e| e.to_string())?;
    // Try parse as ApiResponse<T> first
    if let Ok(wrapper) = serde_json::from_str::<ApiResponse<T>>(&text) {
        return if wrapper.success {
            Ok(wrapper.data)
        } else {
            Err(wrapper.message.unwrap_or_else(|| "Request failed".into()))
        };
    }
    // Fallback: parse T directly
    serde_json::from_str::<T>(&text).map_err(|e| e.to_string())
}

async fn do_refresh(refresh_token: &str) -> Result<String, String> {
    let url = format!("{}/auth/refresh", api_base());
    let body = serde_json::json!({ "refresh_token": refresh_token });
    let builder = Request::post(&url).header("Content-Type", "application/json");
    let req = builder
        .body(body.to_string())
        .map_err(|e| e.to_string())?;
    let resp = req.send().await.map_err(|e| e.to_string())?;
    if !resp.ok() {
        return Err(format!("Refresh failed: {}", resp.status()));
    }
    let text = resp.text().await.map_err(|e| e.to_string())?;
    let token_resp: ApiResponse<TokenResponse> = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    Ok(token_resp.data.access_token)
}
