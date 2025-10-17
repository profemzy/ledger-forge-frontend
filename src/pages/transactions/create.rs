use chrono::NaiveDate;
use leptos::*;
use leptos_router::use_navigate;
use rust_decimal::Decimal;
use crate::utils::format::format_money;
use uuid::Uuid;

use crate::api::{transactions as tx_api, accounts as accounts_api, contacts as contacts_api};
use crate::types::transactions::{CreateLineItemRequest, CreateTransactionRequest, JournalType};
use crate::types::accounts::Account;
use crate::state::{ToastContext, ToastKind};
use crate::types::contacts::Contact;

#[component]
pub fn TransactionCreate() -> impl IntoView {
    // Header fields
    let (date_str, set_date_str) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
    let (reference, set_reference) = create_signal(String::new());
    let (journal, set_journal) = create_signal(None::<JournalType>);
    let (error, set_error) = create_signal(None::<String>);
    let navigate = use_navigate();

    // Accounts for line item selection
    let accounts_res = create_resource(|| (), |_| async move { accounts_api::list_accounts(None, None, Some(false)).await });

    // Optional contact
    let (contact_id, set_contact_id) = create_signal(None::<Uuid>);

    // Load contacts (customers)
    let contacts_res = create_resource(|| (), |_| async move { contacts_api::list_contacts().await });

    // Line items state
    #[derive(Clone)]
    struct Row { account_id: Option<Uuid>, memo: String, debit: String, credit: String }
    let (rows, set_rows) = create_signal::<Vec<Row>>(vec![Row { account_id: None, memo: String::new(), debit: String::new(), credit: String::new() }, Row { account_id: None, memo: String::new(), debit: String::new(), credit: String::new() }]);

    // Helpers
    let add_row = move |_| set_rows.update(|v| v.push(Row { account_id: None, memo: String::new(), debit: String::new(), credit: String::new() }));
    let remove_row = move |i: usize| set_rows.update(|v| { if v.len() > 2 { v.remove(i); } });

    // Totals
    let total_debits = move || {
        rows.get().iter().filter_map(|r| r.debit.replace(",", "").parse::<Decimal>().ok()).fold(Decimal::ZERO, |a, b| a + b)
    };
    let total_credits = move || {
        rows.get().iter().filter_map(|r| r.credit.replace(",", "").parse::<Decimal>().ok()).fold(Decimal::ZERO, |a, b| a + b)
    };

    let toaster = use_context::<ToastContext>();

    let on_submit = create_action(move |_: &()| {
        let date_str = date_str.get();
        let description = description.get();
        let reference = reference.get();
        let journal = journal.get();
        let rows_val = rows.get();
        let navigate = navigate.clone();
        let toaster = toaster.clone();
        async move {
            set_error.set(None);

            // Parse date
            let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|_| "Invalid date (YYYY-MM-DD)".to_string());
            let Ok(tx_date) = date else { set_error.set(date.err()); return; };

            // Validate line items
            if rows_val.len() < 2 { set_error.set(Some("Need at least two line items".into())); return; }
            let mut items: Vec<CreateLineItemRequest> = Vec::new();
            let mut sum_debit = Decimal::ZERO;
            let mut sum_credit = Decimal::ZERO;
            for r in &rows_val {
                let Some(acc) = r.account_id else { set_error.set(Some("All rows need an account".into())); return; };
                let debit = if r.debit.trim().is_empty() { None } else { match r.debit.replace(",", "").parse::<Decimal>() { Ok(d) if d > Decimal::ZERO => { sum_debit += d; Some(d) } _ => { set_error.set(Some("Invalid debit amount".into())); return; } } };
                let credit = if r.credit.trim().is_empty() { None } else { match r.credit.replace(",", "").parse::<Decimal>() { Ok(c) if c > Decimal::ZERO => { sum_credit += c; Some(c) } _ => { set_error.set(Some("Invalid credit amount".into())); return; } } };
                if debit.is_some() && credit.is_some() { set_error.set(Some("A row cannot have both debit and credit".into())); return; }
                if debit.is_none() && credit.is_none() { set_error.set(Some("Each row must have a debit or a credit".into())); return; }
                items.push(CreateLineItemRequest { account_id: acc, description: if r.memo.is_empty() { None } else { Some(r.memo.clone()) }, debit_amount: debit, credit_amount: credit });
            }
            if sum_debit != sum_credit || sum_debit == Decimal::ZERO { set_error.set(Some("Debits must equal credits and be > 0".into())); return; }

            let req = CreateTransactionRequest {
                transaction_date: tx_date,
                description: if description.is_empty() { None } else { Some(description) },
                reference_number: if reference.is_empty() { None } else { Some(reference) },
                contact_id: contact_id.get(),
                company_id: None,
                journal_type: journal,
                line_items: items,
            };

            match tx_api::create_transaction(&req).await {
                Ok(created) => {
                    if let Some(t) = toaster { t.push("Transaction created", ToastKind::Success); }
                    navigate(&format!("/transactions/{}", created.transaction.id), Default::default())
                },
                Err(e) => {
                    if let Some(t) = toaster { t.push(e.clone(), ToastKind::Error); }
                    set_error.set(Some(e))
                },
            }
        }
    });

    view! {
        <div class="p-6 max-w-5xl">
            <h1 class="text-2xl font-semibold mb-4">"New Transaction"</h1>
            {move || error.get().map(|e| view!{ <div class="text-red-600 mb-2">{e}</div> })}

            <form on:submit=move |ev| { ev.prevent_default(); on_submit.dispatch(()); }>
                <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
                    <div>
                        <label class="block mb-1">"Date"</label>
                        <input class="w-full border rounded px-3 py-2" type="date"
                            prop:value=move || date_str.get()
                            on:input=move |e| set_date_str.set(event_target_value(&e))
                        />
                    </div>
                    <div class="md:col-span-2">
                        <label class="block mb-1">"Description"</label>
                        <input class="w-full border rounded px-3 py-2" type="text"
                            prop:value=move || description.get()
                            on:input=move |e| set_description.set(event_target_value(&e))
                        />
                    </div>
                    <div>
                        <label class="block mb-1">"Reference"</label>
                        <input class="w-full border rounded px-3 py-2" type="text"
                            prop:value=move || reference.get()
                            on:input=move |e| set_reference.set(event_target_value(&e))
                        />
                    </div>
                    <div class="md:col-span-2">
                        <label class="block mb-1">"Contact (optional)"</label>
                        <Transition fallback=move || view!{ <div class="text-sm text-gray-500">"Loading contacts..."</div> }>
                            {move || match contacts_res.get() {
                                Some(Ok(list)) => view!{
                                    <select class="w-full border rounded px-3 py-2"
                                        on:change=move |e| {
                                            let v = event_target_value(&e);
                                            if v.is_empty() { set_contact_id.set(None); } else if let Ok(id) = Uuid::parse_str(&v) { set_contact_id.set(Some(id)); }
                                        }
                                    >
                                        <option value="">"None"</option>
                                        {list.into_iter().map(|c: Contact| view!{ <option value={c.id.to_string()}>{c.name}</option> }).collect_view()}
                                    </select>
                                }.into_view(),
                                Some(Err(e)) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                                None => view!{ <div/> }.into_view(),
                            }}
                        </Transition>
                    </div>
                    <div>
                        <label class="block mb-1">"Journal"</label>
                        <select class="w-full border rounded px-3 py-2"
                            on:change=move |e| {
                                let v = event_target_value(&e);
                                let j = match v.as_str() { "General" => Some(JournalType::General), "Sales" => Some(JournalType::Sales), "Cash Receipts" => Some(JournalType::CashReceipts), "Purchases" => Some(JournalType::Purchases), _ => None };
                                set_journal.set(j);
                            }
                        >
                            <option value="">"Select..."</option>
                            <option>"General"</option>
                            <option>"Sales"</option>
                            <option>"Cash Receipts"</option>
                            <option>"Purchases"</option>
                        </select>
                    </div>
                </div>

                <h2 class="text-lg font-semibold mb-2">"Line Items"</h2>
                <div class="overflow-auto">
                    <table class="w-full border-collapse bg-white rounded shadow">
                        <thead>
                            <tr class="text-left border-b">
                                <th class="py-2 px-3 text-gray-600">"Account"</th>
                                <th class="py-2 px-3 text-gray-600">"Memo"</th>
                                <th class="py-2 px-3 text-gray-600">"Debit"</th>
                                <th class="py-2 px-3 text-gray-600">"Credit"</th>
                                <th class="py-2 px-3"></th>
                            </tr>
                        </thead>
                        <tbody>
                            {move || {
                                let list = rows.get();
                                list.into_iter().enumerate().map(|(i, r)| {
                                    view! {
                                        <tr class="border-b">
                                            <td class="py-2 px-3">
                                                <Transition fallback=move || view!{<span class="text-gray-500">"Loading..."</span>}>
                                                    {move || match accounts_res.get() {
                                                        Some(Ok(list)) => view!{
                                                            <select class="border rounded px-2 py-1 w-72"
                                                                on:change=move |e| {
                                                                    let val = event_target_value(&e);
                                                                    set_rows.update(|v| {
                                                                        if let Ok(id) = Uuid::parse_str(&val) { v[i].account_id = Some(id); }
                                                                    });
                                                                }
                                                            >
                                                                <option value="">"Select account"</option>
                                                                {list.into_iter().map(|a: Account| view!{ <option value={a.id.to_string()}>{format!("{} - {}", a.code, a.name)}</option> }).collect_view()}
                                                            </select>
                                                        }.into_view(),
                                                        Some(Err(e)) => view!{ <span class="text-red-600">{e}</span> }.into_view(),
                                                        None => view!{ <span/> }.into_view(),
                                                    }}
                                                </Transition>
                                            </td>
                                            <td class="py-2 px-3">
                                                <input class="border rounded px-2 py-1 w-64" type="text"
                                                    prop:value=r.memo.clone()
                                                    on:input=move |e| set_rows.update(|v| v[i].memo = event_target_value(&e))
                                                />
                                            </td>
                                            <td class="py-2 px-3">
                                                <input class="border rounded px-2 py-1 w-32" type="text" inputmode="decimal" placeholder="0.00"
                                                    prop:value=r.debit.clone()
                                                    on:input=move |e| set_rows.update(|v| { v[i].debit = event_target_value(&e); if !v[i].debit.is_empty() { v[i].credit.clear(); } })
                                                />
                                            </td>
                                            <td class="py-2 px-3">
                                                <input class="border rounded px-2 py-1 w-32" type="text" inputmode="decimal" placeholder="0.00"
                                                    prop:value=r.credit.clone()
                                                    on:input=move |e| set_rows.update(|v| { v[i].credit = event_target_value(&e); if !v[i].credit.is_empty() { v[i].debit.clear(); } })
                                                />
                                            </td>
                                            <td class="py-2 px-3 text-right">
                                                <button class="text-sm text-red-600 hover:underline" type="button" on:click=move |_| remove_row(i)>"Remove"</button>
                                            </td>
                                        </tr>
                                    }
                                }).collect_view()
                            }}
                        </tbody>
                        <tfoot>
                            <tr>
                                <td class="py-2 px-3" colspan="5">
                                    <button class="text-sm text-akowe-blue-600 hover:underline" type="button" on:click=add_row>"+ Add line"</button>
                                </td>
                            </tr>
                        </tfoot>
                    </table>
                </div>

                <div class="mt-4 flex gap-6 items-center">
                    <div>"Total Debits: "<span class="font-mono">{move || format_money(&total_debits())}</span></div>
                    <div>"Total Credits: "<span class="font-mono">{move || format_money(&total_credits())}</span></div>
                    <div class={move || if total_debits() == total_credits() && total_debits() > Decimal::ZERO { "text-green-700" } else { "text-red-700" }}>
                        {move || if total_debits() == total_credits() && total_debits() > Decimal::ZERO { "Balanced" } else { "Unbalanced" }}
                    </div>
                </div>

                <div class="mt-6">
                    <button class="bg-blue-600 text-white px-4 py-2 rounded" type="submit">"Create Transaction"</button>
                </div>
            </form>
        </div>
    }
}
