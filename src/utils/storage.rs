use web_sys::window;

const ACCESS_TOKEN_KEY: &str = "access_token";
const REFRESH_TOKEN_KEY: &str = "refresh_token";

pub fn set_token(token: &str) {
    if let Some(storage) = local_storage() {
        let _ = storage.set_item(ACCESS_TOKEN_KEY, token);
    }
}

pub fn get_token() -> Option<String> {
    local_storage().and_then(|s| s.get_item(ACCESS_TOKEN_KEY).ok().flatten())
}

pub fn remove_token() {
    if let Some(storage) = local_storage() {
        let _ = storage.remove_item(ACCESS_TOKEN_KEY);
    }
}

pub fn set_refresh_token(token: &str) {
    if let Some(storage) = local_storage() {
        let _ = storage.set_item(REFRESH_TOKEN_KEY, token);
    }
}

pub fn get_refresh_token() -> Option<String> {
    local_storage().and_then(|s| s.get_item(REFRESH_TOKEN_KEY).ok().flatten())
}

pub fn remove_refresh_token() {
    if let Some(storage) = local_storage() {
        let _ = storage.remove_item(REFRESH_TOKEN_KEY);
    }
}

fn local_storage() -> Option<web_sys::Storage> {
    window().and_then(|w| w.local_storage().ok().flatten())
}

