use leptos::*;
use leptos_router::*;

use crate::state::AuthContext;

#[component]
pub fn ProtectedRoute() -> impl IntoView {
    let auth = expect_context::<AuthContext>();

    let is_authed = move || {
        auth.is_authenticated() || crate::utils::storage::get_token().is_some()
    };

    view! {
        {move || if is_authed() {
            view! { <Outlet/> }.into_view()
        } else {
            view! { <Redirect path="/login"/> }.into_view()
        }}
    }
}
