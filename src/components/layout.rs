use leptos::*;
use leptos_router::*;
use crate::state::{AuthContext, ToastContext, Toaster};
use crate::api::auth as api_auth;

#[component]
pub fn Layout() -> impl IntoView {
    let location = use_location();
    let is_active = move |prefix: &str| location.pathname.get().starts_with(prefix);
    let auth = expect_context::<AuthContext>();
    let navigate = use_navigate();

    // Provide toast context
    let _toast = ToastContext::provide();

    // Load user if token exists but context is empty
    create_effect(move |_| {
        if auth.user.get().is_none() && crate::utils::storage::get_token().is_some() {
            leptos::spawn_local(async move {
                if let Ok(user) = api_auth::me().await { auth.set_user.set(Some(user)); }
            });
        }
    });

    let logout = move |_| {
        auth.logout();
        navigate("/login", Default::default());
    };

    view! {
        <div class="min-h-screen flex bg-gray-50">
            <aside class="w-64 bg-white border-r p-4 space-y-2">
                <h2 class="text-xl font-semibold mb-4">"Akowe"</h2>
                <nav class="flex flex-col gap-2">
                    <A class={move || if is_active("/dashboard") { "text-akowe-blue-600 font-semibold" } else { "text-gray-700 hover:text-akowe-blue-600" }} href="/dashboard">"Dashboard"</A>
                    <A class={move || if is_active("/accounts") { "text-akowe-blue-600 font-semibold" } else { "text-gray-700 hover:text-akowe-blue-600" }} href="/accounts">"Accounts"</A>
                    <A class={move || if is_active("/transactions") { "text-akowe-blue-600 font-semibold" } else { "text-gray-700 hover:text-akowe-blue-600" }} href="/transactions">"Transactions"</A>
                    <A class={move || if is_active("/invoices") { "text-akowe-blue-600 font-semibold" } else { "text-gray-700 hover:text-akowe-blue-600" }} href="/invoices">"Invoices"</A>
                    <A class={move || if is_active("/payments") { "text-akowe-blue-600 font-semibold" } else { "text-gray-700 hover:text-akowe-blue-600" }} href="/payments">"Payments"</A>
                </nav>
            </aside>
            <div class="flex-1 flex flex-col">
                <header class="flex items-center justify-between p-3 border-b bg-white">
                    <Breadcrumbs />
                    <div class="flex items-center gap-3">
                        {move || auth.get_user().map(|u| view!{ <span class="text-sm text-gray-700">{u.username}</span> })}
                        <button class="text-sm text-gray-600 hover:text-gray-800 underline" on:click=logout>"Logout"</button>
                    </div>
                </header>
                <main class="flex-1 p-6">
                    <Outlet/>
                </main>
                <footer class="p-3 border-t bg-white text-xs text-gray-500 flex items-center justify-between">
                    <span>"LedgerForge / Akowe v"{env!("CARGO_PKG_VERSION")}</span>
                    <span>"All systems nominal"</span>
                </footer>
            </div>
            <Toaster />
        </div>
    }
}

#[component]
fn Breadcrumbs() -> impl IntoView {
    let location = use_location();
    let path = move || location.pathname.get();
    let segments = move || {
        let p = path();
        p.trim_start_matches('/').split('/').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect::<Vec<_>>()
    };
    let label = |seg: &str| -> String {
        match seg {
            "dashboard" => "Dashboard".to_string(),
            "accounts" => "Accounts".to_string(),
            "transactions" => "Transactions".to_string(),
            "invoices" => "Invoices".to_string(),
            "payments" => "Payments".to_string(),
            other => other.to_string(),
        }
    };

    view! {
        <nav class="text-sm text-gray-600">
            <A class="text-akowe-blue-600 hover:underline" href="/">"Home"</A>
            {move || {
                let segs = segments();
                if segs.is_empty() { return view!{ <span/> }.into_view(); }
                let mut acc_path = String::new();
                view!{
                    {segs.iter().enumerate().map(|(i, s)| {
                        acc_path.push('/'); acc_path.push_str(s);
                        let is_last = i == segs.len()-1;
                        let l = label(s);
                        view!{
                            <span>{" / "}</span>
                            {if is_last { view!{ <span class="font-semibold">{l}</span> }.into_view() } else { view!{ <A class="text-akowe-blue-600 hover:underline" href=acc_path.clone()>{l}</A> }.into_view() }}
                        }
                    }).collect_view()}
                }
            }}
        </nav>
    }
}
