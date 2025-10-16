
# Akowe Frontend Implementation Guide

**Framework:** Leptos (Rust + WebAssembly)  
**Project:** LedgerForge Frontend  
**Status:** Ready to Implement  
**Last Updated:** October 7, 2025

---

## 🚀 Quick Start

### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add wasm target
rustup target add wasm32-unknown-unknown

# Install Trunk (build tool for Leptos)
cargo install trunk

# Install Tailwind CSS CLI
npm install -D tailwindcss
npx tailwindcss init
```

### Project Setup

```bash
# Create new Leptos project
cd /Users/t998234/playground/ledger-forge
mkdir -p frontend
cd frontend

# Initialize Cargo project
cargo init --name akowe

# Add dependencies to Cargo.toml
```

---

## 📦 Cargo.toml Configuration

```toml
[package]
name = "akowe"
version = "0.1.0"
edition = "2024"

[dependencies]
# Leptos Framework
leptos = { version = "0.6", features = ["csr"] }
leptos_meta = { version = "0.6" }
leptos_router = { version = "0.6" }

# HTTP Client
reqwest = { version = "0.12", features = ["json"] }
gloo-net = { version = "0.5", features = ["http"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Date/Time
chrono = { version = "0.4", features = ["serde", "wasmbind"] }

# Utilities
uuid = { version = "1.8", features = ["v4", "serde", "js"] }
rust_decimal = { version = "1.36", features = ["serde"] }
web-sys = { version = "0.3", features = ["Storage", "Window"] }
wasm-bindgen = "0.2"

# Logging
console_error_panic_hook = "0.1"
console_log = "1.0"
log = "0.4"

[profile.release]
opt-level = 'z'
lto = true
codegen-units = 1
```

---

## 🏗️ Project Structure

```
frontend/
├── Cargo.toml
├── Trunk.toml
├── index.html
├── tailwind.config.js
├── input.css
├── src/
│   ├── main.rs                 # Entry point
│   ├── app.rs                  # Main app component
│   ├── api/
│   │   ├── mod.rs
│   │   ├── auth.rs            # Auth API calls
│   │   └── client.rs          # HTTP client setup
│   ├── components/
│   │   ├── mod.rs
│   │   ├── auth/
│   │   │   ├── mod.rs
│   │   │   ├── login.rs       # Login form
│   │   │   └── register.rs    # Register form
│   │   ├── dashboard/
│   │   │   ├── mod.rs
│   │   │   ├── stat_card.rs   # Metric cards
│   │   │   ├── chart.rs       # Charts
│   │   │   └── activity.rs    # Activity feed
│   │   ├── layout/
│   │   │   ├── mod.rs
│   │   │   ├── sidebar.rs     # Navigation sidebar
│   │   │   ├── topbar.rs      # Top navigation
│   │   │   └── layout.rs      # Main layout
│   │   └── ui/
│   │       ├── mod.rs
│   │       ├── button.rs      # Button component
│   │       ├── input.rs       # Input component
│   │       └── card.rs        # Card component
│   ├── pages/
│   │   ├── mod.rs
│   │   ├── login.rs           # Login page
│   │   ├── dashboard.rs       # Dashboard page
│   │   └── not_found.rs       # 404 page
│   ├── state/
│   │   ├── mod.rs
│   │   └── auth.rs            # Auth state management
│   ├── types/
│   │   ├── mod.rs
│   │   ├── user.rs            # User types
│   │   └── api.rs             # API response types
│   └── utils/
│       ├── mod.rs
│       └── storage.rs         # LocalStorage helpers
└── public/
    └── assets/
        └── logo.svg
```

---

## 🎨 Tailwind Configuration

**tailwind.config.js:**
```javascript
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html",
  ],
  theme: {
    extend: {
      colors: {
        'akowe-blue': {
          50: '#eff6ff',
          100: '#dbeafe',
          500: '#3b82f6',
          600: '#2563eb',
          700: '#1d4ed8',
        },
        'akowe-green': {
          50: '#f0fdf4',
          500: '#22c55e',
          600: '#16a34a',
        },
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
    },
  },
  plugins: [],
}
```

**input.css:**
```css
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  :root {
    --color-primary: #3b82f6;
    --color-success: #22c55e;
    --color-danger: #ef4444;
  }
  
  body {
    @apply font-sans antialiased;
  }
}

@layer components {
  .btn-primary {
    @apply bg-akowe-blue-600 text-white px-4 py-2 rounded-lg hover:bg-akowe-blue-700 transition-colors;
  }
  
  .card {
    @apply bg-white rounded-lg shadow-sm border border-gray-200 p-6;
  }
}
```

---

## 📝 Implementation Steps

### Step 1: Main Entry Point

**src/main.rs:**
```rust
use leptos::*;

mod app;
mod api;
mod components;
mod pages;
mod state;
mod types;
mod utils;

fn main() {
    // Setup panic hook for better error messages
    console_error_panic_hook::set_once();
    
    // Initialize logging
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");
    
    // Mount the app
    mount_to_body(|| view! { <app::App /> })
}
```

### Step 2: App Component with Routing

**src/app.rs:**
```rust
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::{Dashboard, Login, NotFound};
use crate::state::AuthContext;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    // Global auth state
    let (user, set_user) = create_signal(None);
    provide_context(AuthContext { user, set_user });
    
    view! {
        <Router>
            <Routes>
                <Route path="/" view=|| view! { <Redirect path="/dashboard" /> } />
                <Route path="/login" view=Login />
                <Route path="/dashboard" view=Dashboard />
                <Route path="/*" view=NotFound />
            </Routes>
        </Router>
    }
}
```

### Step 3: Authentication State

**src/state/auth.rs:**
```rust
use leptos::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
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
}
```

### Step 4: API Client

**src/api/client.rs:**
```rust
use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use serde::Serialize;

const API_BASE: &str = "http://localhost:3000/api/v1";

pub async fn post<T: Serialize, R: DeserializeOwned>(
    endpoint: &str,
    body: &T,
) -> Result<R, String> {
    let url = format!("{}{}", API_BASE, endpoint);
    
    let response = Request::post(&url)
        .json(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if response.ok() {
        response
            .json()
            .await
            .map_err(|e| e.to_string())
    } else {
        Err(format!("HTTP {}: {}", response.status(), response.status_text()))
    }
}

pub async fn get_with_auth<R: DeserializeOwned>(
    endpoint: &str,
) -> Result<R, String> {
    let url = format!("{}{}", API_BASE, endpoint);
    let token = crate::utils::storage::get_token()
        .ok_or("No auth token")?;
    
    let response = Request::get(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if response.ok() {
        response
            .json()
            .await
            .map_err(|e| e.to_string())
    } else {
        Err(format!("HTTP {}: {}", response.status(), response.status_text()))
    }
}
```

### Step 5: Login Page

**src/pages/login.rs:**
```rust
use leptos::*;
use leptos_router::*;

use crate::api::auth::login;
use crate::components::ui::{Button, Input};
use crate::state::AuthContext;

#[component]
pub fn Login() -> impl IntoView {
    let auth = use_context::<AuthContext>().expect("AuthContext");
    let navigate = use_navigate();
    
    let (username, set_username) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error, set_error) = create_signal(None::<String>);
    let (loading, set_loading) = create_signal(false);
    
    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match login(&username.get(), &password.get()).await {
                Ok(response) => {
                    // Store token
                    crate::utils::storage::set_token(&response.access_token);
                    
                    // Set user in context
                    auth.set_user.set(Some(response.user));
                    
                    // Navigate to dashboard
                    navigate("/dashboard", Default::default());
                }
                Err(e) => {
                    set_error.set(Some(e));
                    set_loading.set(false);
                }
            }
        });
    };
    
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-akowe-blue-50 to-akowe-blue-100">
            <div class="card max-w-md w-full">
                <div class="text-center mb-8">
                    <h1 class="text-3xl font-bold text-gray-900">✍️ Akowe</h1>
                    <p class="text-gray-600 mt-2">The Documenter</p>
                </div>
                
                <form on:submit=on_submit class="space-y-6">
                    <Input
                        label="Username or Email"
                        type_="text"
                        value=username
                        on_input=move |v| set_username.set(v)
                        required=true
                    />
                    
                    <Input
                        label="Password"
                        type_="password"
                        value=password
                        on_input=move |v| set_password.set(v)
                        required=true
                    />
                    
                    {move || error.get().map(|e| view! {
                        <div class="bg-red-50 text-red-600 p-3 rounded-lg text-sm">
                            {e}
                        </div>
                    })}
                    
                    <Button
                        type_="submit"
                        disabled=loading
                        class="w-full"
                    >
                        {move || if loading.get() { "Signing in..." } else { "Sign In" }}
                    </Button>
                </form>
                
                <p class="text-center text-sm text-gray-600 mt-6">
                    "Don't have an account? "
                    <a href="/register" class="text-akowe-blue-600 hover:underline">
                        "Sign up"
                    </a>
                </p>
            </div>
        </div>
    }
}
```

### Step 6: Dashboard Page

**src/pages/dashboard.rs:**
```rust
use leptos::*;
use leptos_router::*;

use crate::components::dashboard::{StatCard, ActivityFeed};
use crate::components::layout::Layout;
use crate::state::AuthContext;

#[component]
pub fn Dashboard() -> impl IntoView {
    let auth = use_context::<AuthContext>().expect("AuthContext");
    let navigate = use_navigate();
    
    // Redirect if not authenticated
    create_effect(move |_| {
        if !auth.is_authenticated() {
            navigate("/login", Default::default());
        }
    });
    
    view! {
        <Layout>
            <div class="space-y-6">
                <div>
                    <h1 class="text-2xl font-bold text-gray-900">
                        "Welcome back, "
                        {move || auth.user.get().map(|u| u.username).unwrap_or_default()}
                        " 👋"
                    </h1>
                    <p class="text-gray-600 mt-1">"Here's what's happening with your business today."</p>
                </div>
                
                // Stats Grid
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                    <StatCard