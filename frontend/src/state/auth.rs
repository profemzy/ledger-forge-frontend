use leptos::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
}

#[derive(Clone, Copy)]
pub struct AuthContext {
    pub user: ReadSignal<Option<User>>,
    pub set_user: WriteSignal<Option<User>>,
}

impl AuthContext {
    pub fn is_authenticated(&self) -> bool {
        self.user.get().is_some()
    }
    
    pub fn logout(&self) {
        self.set_user.set(None);
        // Clear token from storage
        crate::utils::storage::remove_token();
    }
    
    pub fn get_user(&self) -> Option<User> {
        self.user.get()
    }
}