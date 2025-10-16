use leptos::*;

mod app;
mod api;
mod components;
mod pages;
mod state;
mod types;
mod utils;

fn main() {
    // Setup panic hook for better error messages in browser console
    console_error_panic_hook::set_once();
    
    // Initialize logging
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");
    
    log::info!("🚀 Akowe - The Documenter starting...");
    
    // Mount the app to the body
    mount_to_body(|| view! { <app::App /> })
}