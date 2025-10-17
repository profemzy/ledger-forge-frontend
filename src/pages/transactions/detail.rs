use leptos::*;
use leptos_router::{use_params_map, A, use_navigate};
use uuid::Uuid;
use std::rc::Rc;

use crate::api::transactions as api;
use crate::types::transactions::{TransactionWithLineItems, TransactionStatus};
use crate::state::{ToastContext, ToastKind};
use crate::utils::format::format_money;
use crate::types::accounts::Account;
use crate::api::accounts as accounts_api;
use crate::components::modal::Modal;

#[component]
pub fn TransactionDetail() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned());
    let navigate = use_navigate();

    let tx_res = create_resource(
        move || id(),
        |id_opt| async move {
            let id_str = id_opt.ok_or_else(|| "Missing id".to_string())?;
            let id = Uuid::parse_str(&id_str).map_err(|_| "Invalid id".to_string())?;
            api::get_transaction(id).await
        }
    );

    // Accounts list for mapping account_id -> code-name
    let accounts_res = create_resource(|| (), |_| async move { accounts_api::list_accounts(None, None, Some(true)).await });

    let (msg, set_msg) = create_signal(None::<String>);

    // Actions
    let toaster = use_context::<ToastContext>();
    let toaster_c1 = toaster.clone();

    let post_action = create_action(move |_: &()| {
        let id_opt = id();
        let tx_res = tx_res.clone();
        let set_msg = set_msg.clone();
        let toaster = toaster_c1.clone();
        async move {
            if let Some(id_str) = id_opt { if let Ok(id) = Uuid::parse_str(&id_str) {
                match api::update_transaction_status(id, TransactionStatus::Posted).await {
                    Ok(_) => { if let Some(t) = toaster { t.push("Transaction posted", ToastKind::Success); } },
                    Err(e) => { if let Some(t) = toaster { t.push(e, ToastKind::Error); } },
                }
                tx_res.refetch(); set_msg.set(Some("Posted".into()));
            }}
        }
    });

    let toaster_c2 = toaster.clone();
    let void_action = create_action(move |_: &()| {
        let id_opt = id();
        let tx_res = tx_res.clone();
        let set_msg = set_msg.clone();
        let toaster = toaster_c2.clone();
        async move {
            if let Some(id_str) = id_opt { if let Ok(id) = Uuid::parse_str(&id_str) {
                match api::update_transaction_status(id, TransactionStatus::Void).await {
                    Ok(_) => { if let Some(t) = toaster { t.push("Transaction voided", ToastKind::Success); } },
                    Err(e) => { if let Some(t) = toaster { t.push(e, ToastKind::Error); } },
                }
                tx_res.refetch(); set_msg.set(Some("Voided".into()));
            }}
        }
    });

    let toaster_c3 = toaster.clone();
    let delete_action = create_action(move |_: &()| {
        let id_opt = id();
        let navigate = navigate.clone();
        let toaster = toaster_c3.clone();
        async move {
            if let Some(id_str) = id_opt { if let Ok(id) = Uuid::parse_str(&id_str) {
                match api::delete_transaction(id).await {
                    Ok(_) => { if let Some(t) = toaster { t.push("Transaction deleted", ToastKind::Success); } },
                    Err(e) => { if let Some(t) = toaster { t.push(e, ToastKind::Error); } }
                }
                navigate("/transactions", Default::default());
            }}
        }
    });

    view! {
        <div class="p-6 space-y-4">
            {move || msg.get().map(|m| view!{ <div class="bg-green-100 text-green-800 border border-green-200 px-3 py-2 rounded">{m}</div> })}
            <Transition fallback=move || view! { <div>"Loading transaction..."</div> } >
                {move || match tx_res.get() {
                    Some(Ok(tx)) => {
                        let accounts_opt = accounts_res.get().and_then(|r| r.ok());
                        view! { <TxView tx=tx accounts=accounts_opt post=post_action.clone() void=void_action.clone() del=delete_action.clone()/> }.into_view()
                    }
                    Some(Err(e)) => view! { <div class="text-red-600">{e}</div> }.into_view(),
                    None => view! { <div/> }.into_view(),
                }}
            </Transition>
        </div>
    }
}

#[component]
fn TxView(tx: TransactionWithLineItems, accounts: Option<Vec<Account>>, post: Action<(), ()>, void: Action<(), ()>, del: Action<(), ()>) -> impl IntoView {
    let can_post = matches!(tx.transaction.status, TransactionStatus::Draft);
    let can_void = matches!(tx.transaction.status, TransactionStatus::Posted);
    let can_delete = matches!(tx.transaction.status, TransactionStatus::Draft);

    // Build lookup map for accounts
    let lookup = accounts.map(|v| v.into_iter().map(|a| (a.id, format!("{} - {}", a.code, a.name))).collect::<std::collections::HashMap<_, _>>());

    let (show_delete, set_show_delete) = create_signal(false);

    view! {
        <div class="space-y-4">
            <div class="text-sm text-gray-600"><A class="text-akowe-blue-600 hover:underline" href="/transactions">"Transactions"</A> " / " {tx.transaction.id.to_string()}</div>
            <div class="bg-white rounded shadow p-4">
                <div class="flex items-center justify-between">
                    <div>
                        <div class="text-xl font-semibold">{tx.transaction.description.clone().unwrap_or_else(|| "—".into())}</div>
                        <div class="text-gray-600">{"Date: "}{tx.transaction.transaction_date.to_string()} {" • Ref: "}{tx.transaction.reference_number.clone().unwrap_or_else(|| "—".into())}</div>
                        <div class="mt-1">
                            <span class="text-xs px-2 py-1 rounded bg-gray-100 text-gray-700 border">{"Status: "}{match tx.transaction.status { TransactionStatus::Draft => "draft", TransactionStatus::Posted => "posted", TransactionStatus::Void => "void" }}</span>
                            {tx.transaction.journal_type.as_ref().map(|j| view!{ <span class="ml-2 text-xs px-2 py-1 rounded bg-blue-100 text-blue-700 border">{format!("Journal: {:?}", j)}</span> }.into_view()).unwrap_or_else(|| view!{ <span/> }.into_view())}
                        </div>
                    </div>
                    <div class="space-x-2">
                        { if can_post { view!{ <button class="bg-green-600 text-white px-3 py-2 rounded" on:click=move |_| post.dispatch(())>"Post"</button> }.into_view() } else { view!{ <span/> }.into_view() } }
                        { if can_void { view!{ <button class="bg-red-600 text-white px-3 py-2 rounded" on:click=move |_| void.dispatch(())>"Void"</button> }.into_view() } else { view!{ <span/> }.into_view() } }
                        { if can_delete { view!{ <button class="bg-gray-600 text-white px-3 py-2 rounded" on:click=move |_| set_show_delete.set(true)>"Delete"</button> }.into_view() } else { view!{ <span/> }.into_view() } }
                    </div>
                </div>
            </div>

            <div class="bg-white rounded shadow">
                <table class="w-full border-collapse">
                    <thead>
                        <tr class="text-left border-b">
                            <th class="py-2 px-3 text-gray-600">"Account"</th>
                            <th class="py-2 px-3 text-gray-600">"Memo"</th>
                            <th class="py-2 px-3 text-gray-600">"Debit"</th>
                            <th class="py-2 px-3 text-gray-600">"Credit"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {let lookup = lookup.clone(); tx.line_items.iter().map(move |li| {
                            let label = lookup.as_ref().and_then(|m| m.get(&li.account_id)).cloned().unwrap_or_else(|| li.account_id.to_string());
                            view!{
                                <tr class="border-b">
                                    <td class="py-2 px-3"><A class="text-akowe-blue-600 hover:underline" href=format!("/accounts/{}", li.account_id)>{label}</A></td>
                                    <td class="py-2 px-3">{li.description.clone().unwrap_or_else(|| "—".into())}</td>
                                <td class="py-2 px-3">{format_money(&li.debit_amount)}</td>
                                <td class="py-2 px-3">{format_money(&li.credit_amount)}</td>
                                </tr>
                            }
                        }).collect_view()}
                    </tbody>
                </table>
                { // Totals
                    let total_debits = tx.line_items.iter().fold(rust_decimal::Decimal::ZERO, |acc, li| acc + li.debit_amount);
                    let total_credits = tx.line_items.iter().fold(rust_decimal::Decimal::ZERO, |acc, li| acc + li.credit_amount);
                    view!{
                        <div class="flex justify-end gap-6 p-4">
                            <div>"Total Debits: "<span class="font-mono">{format_money(&total_debits)}</span></div>
                            <div>"Total Credits: "<span class="font-mono">{format_money(&total_credits)}</span></div>
                        </div>
                    }
                }
            </div>
            <Modal show=show_delete on_close=Callback::new(move |_| set_show_delete.set(false)) title="Delete Transaction".to_string()
                actions=Rc::new(move || view!{
                    <button class="px-3 py-1 rounded border" on:click=move |_| set_show_delete.set(false)>"Cancel"</button>
                    <button class="px-3 py-1 rounded bg-red-600 text-white" on:click=move |_| { del.dispatch(()); set_show_delete.set(false); }>"Delete"</button>
                })
            >
                <p>{"Delete this draft transaction? This cannot be undone."}</p>
            </Modal>
        </div>
    }
}
