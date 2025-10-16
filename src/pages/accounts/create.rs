use leptos::*;
use leptos_router::use_navigate;

use crate::api::accounts as api;
use crate::types::accounts::{Account, AccountType, CreateAccountRequest};
use crate::state::{ToastContext, ToastKind};

#[component]
pub fn AccountCreate() -> impl IntoView {
    let (code, set_code) = create_signal(String::new());
    let (name, set_name) = create_signal(String::new());
    let (acc_type, set_acc_type) = create_signal(AccountType::Asset);
    let (error, set_error) = create_signal(None::<String>);
    let (parent_id, set_parent_id) = create_signal(None::<uuid::Uuid>);

    // Load all accounts for parent selection (includes inactive)
    let accounts_res = create_resource(|| (), |_| async move { api::list_accounts(None, None, Some(true)).await });
    let navigate = use_navigate();

    let toaster = use_context::<ToastContext>();

    let on_submit = create_action(move |_: &()| {
        let code = code.get();
        let name = name.get();
        let acc_type = acc_type.get();
        let navigate = navigate.clone();
        let toaster = toaster.clone();
        async move {
            set_error.set(None);
            if code.trim().is_empty() || name.trim().is_empty() {
                set_error.set(Some("Code and Name are required".to_string()));
                return;
            }

            let req = CreateAccountRequest {
                code,
                name,
                account_type: acc_type,
                parent_account_id: parent_id.get(),
                company_id: None,
            };

            match api::create_account(&req).await {
                Ok(account) => {
                    if let Some(t) = toaster { t.push("Account created", ToastKind::Success); }
                    navigate(&format!("/accounts/{}", account.id), Default::default())
                },
                Err(e) => {
                    if let Some(t) = toaster { t.push(e.clone(), ToastKind::Error); }
                    set_error.set(Some(e))
                },
            }
        }
    });

    view! {
        <div class="p-6 max-w-xl">
            <h1 class="text-2xl font-semibold mb-4">"New Account"</h1>
            {move || error.get().map(|e| view!{ <div class="text-red-600 mb-2">{e}</div> })}
            <form on:submit=move |ev| { ev.prevent_default(); on_submit.dispatch(()); }>
                <label class="block mb-1">"Code"</label>
                <input class="w-full border rounded px-3 py-2 mb-4" type="text"
                    prop:value=move || code.get()
                    on:input=move |e| set_code.set(event_target_value(&e))
                />

                <label class="block mb-1">"Name"</label>
                <input class="w-full border rounded px-3 py-2 mb-4" type="text"
                    prop:value=move || name.get()
                    on:input=move |e| set_name.set(event_target_value(&e))
                />

                <label class="block mb-1">"Account Type"</label>
                <select class="w-full border rounded px-3 py-2 mb-6"
                    on:change=move |e| {
                        let val = event_target_value(&e);
                        let t = match val.as_str() {
                            "Asset" => AccountType::Asset,
                            "Liability" => AccountType::Liability,
                            "Equity" => AccountType::Equity,
                            "Revenue" => AccountType::Revenue,
                            "Expense" => AccountType::Expense,
                            _ => AccountType::Asset,
                        };
                        set_acc_type.set(t);
                    }
                >
                    <option>"Asset"</option>
                    <option>"Liability"</option>
                    <option>"Equity"</option>
                    <option>"Revenue"</option>
                    <option>"Expense"</option>
                </select>

                <label class="block mb-1">"Parent Account (optional)"</label>
                <Transition fallback=move || view! { <div class="text-sm text-gray-500 mb-6">"Loading accounts..."</div> }>
                    {move || match accounts_res.get() {
                        Some(Ok(list)) => view! {
                            <select class="w-full border rounded px-3 py-2 mb-6"
                                on:change=move |e| {
                                    let val = event_target_value(&e);
                                    if val.is_empty() { set_parent_id.set(None); } else if let Ok(id) = uuid::Uuid::parse_str(&val) { set_parent_id.set(Some(id)); }
                                }
                            >
                                <option value="">"None"</option>
                                {list.into_iter().map(|a: Account| view! { <option value={a.id.to_string()}>{format!("{} - {}", a.code, a.name)}</option> }).collect_view()}
                            </select>
                        }.into_view(),
                        Some(Err(e)) => view! { <div class="text-red-600 mb-6">{e}</div> }.into_view(),
                        None => view! { <div/> }.into_view(),
                    }}
                </Transition>

                <button class="bg-blue-600 text-white px-4 py-2 rounded" type="submit">"Create"</button>
            </form>
        </div>
    }
}
