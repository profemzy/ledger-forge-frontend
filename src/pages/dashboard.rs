use leptos::*;
use leptos_router::*;

use crate::state::AuthContext;

#[component]
pub fn Dashboard() -> impl IntoView {
    let navigate = use_navigate();
    let auth = expect_context::<AuthContext>();

    create_effect(move |_| {
        if !auth.is_authenticated() {
            navigate("/login", Default::default());
        }
    });

    view! {
        <div class="p-6">
            <h1 class="text-2xl font-semibold">"Dashboard"</h1>
            {move || auth.get_user().map(|u| view!{
                <p class="mt-2">{"Welcome, "}{u.username}</p>
            })}
        </div>
    }
}

