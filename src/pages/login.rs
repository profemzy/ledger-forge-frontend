use leptos::*;
use leptos_router::use_navigate;

use crate::api::auth as api_auth;
use crate::state::{AuthContext, ToastContext, ToastKind, Toaster};

#[component]
pub fn Login() -> impl IntoView {
    let (username, set_username) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error, set_error) = create_signal(None::<String>);
    let navigate = use_navigate();
    let auth = expect_context::<AuthContext>();

    // Provide toast context for this public route
    let toaster = ToastContext::provide();

    let on_submit = create_action(move |_: &()| {
        let username = username.get();
        let password = password.get();
        let navigate = navigate.clone();
        let toaster = toaster.clone();
        async move {
            set_error.set(None);
            match api_auth::login(username, password).await {
                Ok(resp) => {
                    auth.set_user.set(Some(resp.user));
                    toaster.push("Signed in", ToastKind::Success);
                    navigate("/dashboard", Default::default());
                }
                Err(e) => { toaster.push(e.clone(), ToastKind::Error); set_error.set(Some(e)); }
            }
        }
    });

    view! {
        <div class="container mx-auto max-w-md mt-24 p-6 border rounded">
            <h1 class="text-2xl font-semibold mb-4">"Login"</h1>
            {move || error.get().map(|e| view!{ <div class="text-red-600 mb-2">{e}</div> })}
            <form on:submit=move |ev| { ev.prevent_default(); on_submit.dispatch(()); }>
                <label class="block mb-2">"Username"</label>
                <input class="w-full border rounded px-3 py-2 mb-4" type="text"
                    prop:value=move || username.get()
                    on:input=move |e| set_username.set(event_target_value(&e))
                />
                <label class="block mb-2">"Password"</label>
                <input class="w-full border rounded px-3 py-2 mb-4" type="password"
                    prop:value=move || password.get()
                    on:input=move |e| set_password.set(event_target_value(&e))
                />
                <button class="bg-blue-600 text-white px-4 py-2 rounded" type="submit">"Sign In"</button>
            </form>
        </div>
        <Toaster />
    }
}
