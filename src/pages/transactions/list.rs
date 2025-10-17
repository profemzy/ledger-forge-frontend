use leptos::*;
use leptos_router::A;

use crate::api::transactions as api;
use crate::components::ui::{ButtonLink, Table};
use crate::types::transactions::{Transaction, TransactionStatus};
use uuid::Uuid;

#[component]
pub fn TransactionsList() -> impl IntoView {
    let (status, set_status) = create_signal(None::<TransactionStatus>);
    let (limit, set_limit) = create_signal(Some(50_i64));
    let (query, set_query) = create_signal(String::new());
    let (company_id_str, set_company_id_str) = create_signal(String::new());

    let txs = create_resource(
        move || (status.get(), limit.get(), company_id_str.get()),
        |(s, l, cid_str)| async move {
            let company_id = if cid_str.trim().is_empty() { None } else { Uuid::parse_str(&cid_str).ok() };
            api::list_transactions(s, company_id, l).await
        },
    );

    view! {
        <div class="p-6">
            <div class="flex items-center justify-between mb-4">
                <h1 class="text-2xl font-semibold">"Transactions"</h1>
                <ButtonLink href="/transactions/new" variant="primary">{"New Transaction"}</ButtonLink>
            </div>

            <div class="flex gap-4 mb-4 items-center">
                <input
                    class="border rounded px-3 py-2 w-72"
                    type="text"
                    placeholder="Search description or reference..."
                    prop:value=move || query.get()
                    on:input=move |e| set_query.set(event_target_value(&e))
                />
                <input
                    class="border rounded px-3 py-2 w-72"
                    type="text"
                    placeholder="Filter by Company ID (UUID)"
                    prop:value=move || company_id_str.get()
                    on:input=move |e| set_company_id_str.set(event_target_value(&e))
                />
                <select class="border rounded px-2 py-1"
                    on:change=move |e| {
                        let v = event_target_value(&e);
                        let s = match v.as_str() {
                            "draft" => Some(TransactionStatus::Draft),
                            "posted" => Some(TransactionStatus::Posted),
                            "void" => Some(TransactionStatus::Void),
                            _ => None,
                        };
                        set_status.set(s);
                    }
                >
                    <option value="">"All Statuses"</option>
                    <option value="draft">"Draft"</option>
                    <option value="posted">"Posted"</option>
                    <option value="void">"Void"</option>
                </select>

                <select class="border rounded px-2 py-1"
                    on:change=move |e| {
                        let v = event_target_value(&e);
                        let l = v.parse::<i64>().unwrap_or(50);
                        set_limit.set(Some(l));
                    }
                >
                    <option value="25">"25"</option>
                    <option value="50" selected>"50"</option>
                    <option value="100">"100"</option>
                </select>
            </div>

            <Transition fallback=move || view!{ <div>"Loading transactions..."</div> }>
                {move || match txs.get() {
                    Some(Ok(list)) => {
                        let q = query.get().to_lowercase();
                        let filtered = if q.is_empty() { list } else {
                            list.into_iter().filter(|t| t.description.as_deref().unwrap_or("").to_lowercase().contains(&q) || t.reference_number.as_deref().unwrap_or("").to_lowercase().contains(&q)).collect()
                        };
                        if filtered.is_empty() {
                            view!{ <div class="text-gray-600">"No transactions found."</div> }.into_view()
                        } else {
                            view!{ <TxTable txs=filtered/> }.into_view()
                        }
                    }
                    Some(Err(e)) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                    None => view!{ <div/> }.into_view(),
                }}
            </Transition>
        </div>
    }
}

#[component]
fn TxTable(txs: Vec<Transaction>) -> impl IntoView {
    view! {
        <Table>
            <thead>
                <tr class="text-left border-b bg-gray-50">
                    <th class="py-2 px-3 text-gray-600">"Date"</th>
                    <th class="py-2 px-3 text-gray-600">"Description"</th>
                    <th class="py-2 px-3 text-gray-600">"Reference"</th>
                    <th class="py-2 px-3 text-gray-600">"Status"</th>
                </tr>
            </thead>
            <tbody>
                {txs.into_iter().map(|t| view! {
                    <tr class="border-b hover:bg-gray-50 dark:hover:bg-gray-800">
                        <td class="py-2 px-3">{t.transaction_date.to_string()}</td>
                        <td class="py-2 px-3"><A class="text-akowe-blue-600 hover:underline" href=format!("/transactions/{}", t.id)>{t.description.unwrap_or_else(|| "—".into())}</A></td>
                        <td class="py-2 px-3 font-mono text-sm">{t.reference_number.unwrap_or_else(|| "—".into())}</td>
                        <td class="py-2 px-3">
                            {match t.status {
                                TransactionStatus::Draft => view!{ <span class="text-xs px-2 py-1 rounded bg-gray-100 text-gray-700 border">"draft"</span> }.into_view(),
                                TransactionStatus::Posted => view!{ <span class="text-xs px-2 py-1 rounded bg-green-100 text-green-700 border">"posted"</span> }.into_view(),
                                TransactionStatus::Void => view!{ <span class="text-xs px-2 py-1 rounded bg-red-100 text-red-700 border">"void"</span> }.into_view(),
                            }}
                        </td>
                    </tr>
                }).collect_view()}
            </tbody>
        </Table>
    }
}
