use leptos::*;
use leptos_router::{use_params_map, A};
use uuid::Uuid;

use crate::api::accounts as api;
use crate::types::accounts::{Account, UpdateAccountRequest};
use crate::state::{ToastContext, ToastKind};

#[component]
pub fn AccountDetail() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned());

    let account_res = create_resource(
        move || id(),
        |id_opt| async move {
            let id_str = id_opt.ok_or_else(|| "Missing account id".to_string())?;
            let id = Uuid::parse_str(&id_str).map_err(|_| "Invalid account id".to_string())?;
            api::get_account(id).await
        }
    );

    let balance_res = create_resource(
        move || id(),
        |id_opt| async move {
            let id_str = id_opt.ok_or_else(|| "Missing account id".to_string())?;
            let id = Uuid::parse_str(&id_str).map_err(|_| "Invalid account id".to_string())?;
            api::get_account_balance(id).await
        }
    );

    let hierarchy_res = create_resource(
        move || id(),
        |id_opt| async move {
            let id_str = id_opt.ok_or_else(|| "Missing account id".to_string())?;
            let id = Uuid::parse_str(&id_str).map_err(|_| "Invalid account id".to_string())?;
            api::get_account_hierarchy(id).await
        }
    );

    // Local editing state for name
    let toaster = use_context::<ToastContext>();
    let toaster_clone1 = toaster.clone();
    let (edit_name, set_edit_name) = create_signal(String::new());
    create_effect(move |_| {
        if let Some(Ok(acc)) = account_res.get() {
            set_edit_name.set(acc.name.clone());
        }
    });

    // Save rename
    let save_action = create_action(move |_: &()| {
        let id_opt = id();
        let name = edit_name.get();
        let account_res = account_res.clone();
        let toaster = toaster_clone1.clone();
        async move {
            if let Some(id_str) = id_opt {
                if let Ok(id) = Uuid::parse_str(&id_str) {
                    let req = UpdateAccountRequest { name: Some(name), is_active: None };
                    match api::update_account(id, &req).await {
                        Ok(_) => { if let Some(t) = toaster.clone() { t.push("Account saved", ToastKind::Success); } }
                        Err(e) => { if let Some(t) = toaster.clone() { t.push(e, ToastKind::Error); } }
                    }
                    account_res.refetch();
                }
            }
        }
    });

    // Toggle active status
    let toaster_clone2 = toaster.clone();
    let toggle_active = create_action(move |active: &bool| {
        let id_opt = id();
        let make_active = *active;
        let account_res = account_res.clone();
        let toaster = toaster_clone2.clone();
        async move {
            if let Some(id_str) = id_opt {
                if let Ok(id) = Uuid::parse_str(&id_str) {
                    let req = UpdateAccountRequest { name: None, is_active: Some(make_active) };
                    match api::update_account(id, &req).await {
                        Ok(_) => { if let Some(t) = toaster.clone() { t.push(if make_active {"Account activated"} else {"Account deactivated"}, ToastKind::Success); } }
                        Err(e) => { if let Some(t) = toaster.clone() { t.push(e, ToastKind::Error); } }
                    }
                    account_res.refetch();
                }
            }
        }
    });

    let (msg, set_msg) = create_signal(None::<String>);

    view! {
        <div class="p-6 space-y-4">
            <div class="text-sm text-gray-600"><A class="text-akowe-blue-600 hover:underline" href="/accounts">"Accounts"</A> " / " {move || account_res.get().map(|r| r.ok().map(|a| a.name).unwrap_or_default()).unwrap_or_default()}</div>
            {move || msg.get().map(|m| view!{ <div class="bg-green-100 text-green-800 border border-green-200 px-3 py-2 rounded">{m}</div> })}
            <Transition fallback=move || view! { <div>"Loading account..."</div> }>
                {move || match account_res.get() {
                    Some(Ok(account)) => view! { <AccountView account=account/> }.into_view(),
                    Some(Err(e)) => view! { <div class="text-red-600">{e}</div> }.into_view(),
                    None => view! { <div/> }.into_view(),
                }}
            </Transition>

            <div class="mt-4">
                <h2 class="text-lg font-semibold">"Balance"</h2>
                <Transition fallback=move || view! { <div>"Calculating..."</div> }>
                    {move || match balance_res.get() {
                        Some(Ok(bal)) => view! { <div class="mt-1">{format!("{}", bal)}</div> }.into_view(),
                        Some(Err(e)) => view! { <div class="text-red-600">{e}</div> }.into_view(),
                        None => view! { <div/> }.into_view(),
                    }}
                </Transition>
            </div>

            <div class="mt-6 space-y-4">
                <div>
                    <h2 class="text-lg font-semibold">"Edit Account"</h2>
                    <div class="mt-2 flex gap-2">
                        <input class="border rounded px-3 py-2 w-80" type="text"
                            prop:value=move || edit_name.get()
                            on:input=move |e| set_edit_name.set(event_target_value(&e))
                        />
                        <button class="bg-akowe-blue-600 text-white px-4 py-2 rounded" on:click=move |_| { set_msg.set(None); save_action.dispatch(()); set_msg.set(Some("Saved".into())); }>"Save"</button>
                    </div>
                </div>
                <div>
                    <h2 class="text-lg font-semibold">"Status"</h2>
                    <Show when=move || matches!(account_res.get(), Some(Ok(acc)) if !acc.is_active)>
                        <button class="bg-green-600 text-white px-4 py-2 rounded" on:click=move |_| { set_msg.set(None); toggle_active.dispatch(true); set_msg.set(Some("Activated".into())); }>"Activate"</button>
                    </Show>
                    <Show when=move || matches!(account_res.get(), Some(Ok(acc)) if acc.is_active)>
                        <button class="bg-red-600 text-white px-4 py-2 rounded" on:click=move |_| {
                            if web_sys::window().and_then(|w| w.confirm_with_message("Deactivate this account?").ok()).unwrap_or(false) {
                                set_msg.set(None);
                                toggle_active.dispatch(false);
                                set_msg.set(Some("Deactivated".into()));
                            }
                        }>"Deactivate"</button>
                    </Show>
                </div>
            </div>

            <div class="mt-6">
                <h2 class="text-lg font-semibold">"Hierarchy"</h2>
                <Transition fallback=move || view! { <div>"Loading hierarchy..."</div> }>
                    {move || match hierarchy_res.get() {
                        Some(Ok(h)) => {
                            let parent_owned = h.parent.as_ref().map(|p| (p.id, p.name.clone()));
                            let child_owned: Vec<(uuid::Uuid, String)> = h.children.iter().map(|c| (c.id, c.name.clone())).collect();
                            let count = child_owned.len();
                            view! {
                                <div class="mt-2 space-y-2">
                                    <div>
                                        <span class="font-medium">"Parent: "</span>
                                        {parent_owned.map(|(id, name)| view!{ <A class="text-akowe-blue-600 hover:underline" href=format!("/accounts/{}", id)>{name}</A> }.into_view()).unwrap_or_else(|| view!{ <span>"—"</span> }.into_view())}
                                    </div>
                                    <div>
                                        <span class="font-medium">"Children ("{count}"):"</span>
                                        <ul class="list-disc list-inside">
                                            {child_owned.into_iter().map(|(id, name)| view! { <li><A class="text-akowe-blue-600 hover:underline" href=format!("/accounts/{}", id)>{name}</A></li> }).collect_view()}
                                        </ul>
                                    </div>
                                </div>
                            }.into_view()
                        },
                        Some(Err(e)) => view! { <div class="text-red-600">{e}</div> }.into_view(),
                        None => view! { <div/> }.into_view(),
                    }}
                </Transition>
            </div>
        </div>
    }
}

#[component]
fn AccountView(account: Account) -> impl IntoView {
    view! {
        <div>
            <h1 class="text-2xl font-semibold">{account.name.clone()}</h1>
            <p class="text-gray-600">{"Code: "}{account.code.clone()} {" • Type: "}{format!("{:?}", account.account_type)}</p>
            <p class="mt-2">{"Status: "}{if account.is_active {"Active"} else {"Inactive"}}</p>
        </div>
    }
}
