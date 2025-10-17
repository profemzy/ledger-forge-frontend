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

    // Dark mode toggle
    let (dark, set_dark) = create_signal(false);
    create_effect(move |_| {
        // initialize from localStorage
        if let Some(win) = web_sys::window() {
            if let Ok(Some(storage)) = win.local_storage() {
                if let Ok(Some(val)) = storage.get_item("theme") { set_dark.set(val == "dark"); }
            }
        }
        apply_dark_class(dark.get());
    });

    let toggle_dark = move |_| {
        let next = !dark.get();
        set_dark.set(next);
        apply_dark_class(next);
        if let Some(win) = web_sys::window() {
            if let Ok(Some(storage)) = win.local_storage() {
                let _ = storage.set_item("theme", if next { "dark" } else { "light" });
            }
        }
    };

    fn apply_dark_class(is_dark: bool) {
        if let Some(win) = web_sys::window() {
            if let Some(doc) = win.document() {
                if let Some(root) = doc.document_element() {
                    let _ = if is_dark { root.class_list().add_1("dark") } else { root.class_list().remove_1("dark") };
                }
            }
        }
    }

    // Sidebar open state (mobile)
    let (nav_open, set_nav_open) = create_signal(false);

    let close_nav = move |_| set_nav_open.set(false);
    let toggle_nav = move |_| set_nav_open.set(!nav_open.get());

    view! {
        <div class="min-h-screen flex bg-gray-50 dark:bg-gray-950">
            // Overlay for mobile when sidebar open
            {move || if nav_open.get() { view!{ <div class="fixed inset-0 bg-black/30 md:hidden z-20" on:click=close_nav/> }.into_view() } else { view!{ <span/> }.into_view() }}
            <aside class={move || format!("{} {}",
                    "fixed inset-y-0 left-0 z-30 transform transition-transform duration-200 w-64 bg-white dark:bg-gray-900 border-r dark:border-gray-800 p-4 space-y-2 md:relative",
                    if nav_open.get() { "translate-x-0" } else { "-translate-x-full md:translate-x-0" }
                )}
            >
                <div class="flex items-center gap-2 mb-4">
                    <div class="w-2 h-5 bg-akowe-blue-600 rounded-sm"></div>
                    <h2 class="text-xl font-semibold tracking-tight">"Akowe"</h2>
                </div>
                <nav class="flex flex-col gap-1">
                    <A class={move || if is_active("/dashboard") { "block px-3 py-2 rounded bg-akowe-blue-50 text-akowe-blue-700 font-medium dark:bg-blue-950 dark:text-blue-100" } else { "block px-3 py-2 rounded text-gray-700 hover:bg-gray-100 dark:text-gray-200 dark:hover:bg-gray-800" }} href="/dashboard">"Dashboard"</A>
                    <A class={move || if is_active("/accounts") { "block px-3 py-2 rounded bg-akowe-blue-50 text-akowe-blue-700 font-medium dark:bg-blue-950 dark:text-blue-100" } else { "block px-3 py-2 rounded text-gray-700 hover:bg-gray-100 dark:text-gray-200 dark:hover:bg-gray-800" }} href="/accounts">"Accounts"</A>
                    <A class={move || if is_active("/transactions") { "block px-3 py-2 rounded bg-akowe-blue-50 text-akowe-blue-700 font-medium dark:bg-blue-950 dark:text-blue-100" } else { "block px-3 py-2 rounded text-gray-700 hover:bg-gray-100 dark:text-gray-200 dark:hover:bg-gray-800" }} href="/transactions">"Transactions"</A>
                    <A class={move || if is_active("/invoices") { "block px-3 py-2 rounded bg-akowe-blue-50 text-akowe-blue-700 font-medium dark:bg-blue-950 dark:text-blue-100" } else { "block px-3 py-2 rounded text-gray-700 hover:bg-gray-100 dark:text-gray-200 dark:hover:bg-gray-800" }} href="/invoices">"Invoices"</A>
                    <A class={move || if is_active("/payments") { "block px-3 py-2 rounded bg-akowe-blue-50 text-akowe-blue-700 font-medium dark:bg-blue-950 dark:text-blue-100" } else { "block px-3 py-2 rounded text-gray-700 hover:bg-gray-100 dark:text-gray-200 dark:hover:bg-gray-800" }} href="/payments">"Payments"</A>
                    <A class={move || if is_active("/bills") { "block px-3 py-2 rounded bg-akowe-blue-50 text-akowe-blue-700 font-medium dark:bg-blue-950 dark:text-blue-100" } else { "block px-3 py-2 rounded text-gray-700 hover:bg-gray-100 dark:text-gray-200 dark:hover:bg-gray-800" }} href="/bills">"Bills"</A>
                </nav>
            </aside>
            <div class="flex-1 flex flex-col">
                <header class="sticky top-0 z-20 flex items-center justify-between p-3 border-b border-gray-200 dark:border-gray-800 bg-white/80 dark:bg-gray-900/80 backdrop-blur">
                    <div class="flex items-center gap-2">
                        <button class="md:hidden inline-flex items-center justify-center rounded border px-2 py-1 text-gray-700 hover:text-gray-900 dark:text-gray-200" on:click=toggle_nav aria-label="Open navigation">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-5 h-5"><path d="M3.75 5.25h16.5v1.5H3.75v-1.5Zm0 6h16.5v1.5H3.75v-1.5Zm0 6h16.5v1.5H3.75v-1.5Z"/></svg>
                        </button>
                        <Breadcrumbs />
                    </div>
                    <div class="flex items-center gap-3">
                        <button class="inline-flex items-center justify-center rounded border px-2 py-1 text-gray-700 hover:text-gray-900 dark:text-gray-200" on:click=toggle_dark aria-label="Toggle dark mode">
                            {move || if dark.get() {
                                view!{ <span>"🌙"</span> }.into_view()
                            } else {
                                view!{ <span>"☀️"</span> }.into_view()
                            }}
                        </button>
                        {move || auth.get_user().map(|u| view!{ <span class="text-sm text-gray-700">{u.username}</span> })}
                        <button class="text-sm text-gray-600 hover:text-gray-900 dark:text-gray-200 border px-3 py-1 rounded" on:click=logout>"Logout"</button>
                    </div>
                </header>
                <main class="flex-1 p-6">
                    <Outlet/>
                </main>
                <footer class="p-3 border-t border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900 text-xs text-gray-500 dark:text-gray-400 flex items-center justify-between">
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
