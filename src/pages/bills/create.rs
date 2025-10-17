use chrono::NaiveDate;
use leptos::*;
use leptos_router::use_navigate;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::api::{bills as bill_api, contacts as contacts_api, accounts as accounts_api};
use crate::types::bills::{CreateBillRequest, CreateBillLineItemRequest};
use crate::types::accounts::{Account, AccountType};
use crate::types::contacts::Contact;
use crate::utils::format::mask_money_input;
use crate::state::{ToastContext, ToastKind};
use crate::utils::format::format_money;

#[component]
pub fn BillCreate() -> impl IntoView {
    let (bill_number, set_bill_number) = create_signal(String::new());
    let (vendor_id, set_vendor_id) = create_signal(None::<Uuid>);
    let (bill_date_str, set_bill_date_str) = create_signal(String::new());
    let (due_date_str, set_due_date_str) = create_signal(String::new());
    let (memo, set_memo) = create_signal(String::new());
    let (error, set_error) = create_signal(None::<String>);
    let navigate = use_navigate();
    let toaster = use_context::<ToastContext>();

    let vendors_res = create_resource(|| (), |_| async move { contacts_api::list_vendors().await });
    let expense_accounts_res = create_resource(|| (), |_| async move { accounts_api::list_accounts(Some(AccountType::Expense), None, Some(false)).await });
    let customers_res = create_resource(|| (), |_| async move { contacts_api::list_customers().await });

    #[derive(Clone)]
    struct Row { desc: String, amount: String, expense_acc: Option<Uuid>, billable: bool, customer: Option<Uuid> }
    let (rows, set_rows) = create_signal::<Vec<Row>>(vec![Row { desc: String::new(), amount: String::new(), expense_acc: None, billable: false, customer: None }]);
    let add_row = move |_| set_rows.update(|v| v.push(Row { desc: String::new(), amount: String::new(), expense_acc: None, billable: false, customer: None }));
    let remove_row = move |i: usize| set_rows.update(|v| { if v.len() > 1 { v.remove(i); } });

    let total_amount = move || {
        rows.get().iter().filter_map(|r| r.amount.replace(",", "").parse::<Decimal>().ok()).fold(Decimal::ZERO, |a, b| a + b)
    };

    let on_submit = create_action(move |_: &()| {
        let bill_number = bill_number.get();
        let vendor = vendor_id.get();
        let bill_date = bill_date_str.get();
        let due_date = due_date_str.get();
        let memo = memo.get();
        let rows_val = rows.get();
        let navigate = navigate.clone();
        let toaster = toaster.clone();
        async move {
            set_error.set(None);
            let Some(vendor_id) = vendor else { set_error.set(Some("Vendor is required".into())); return; };
            let bill_date = NaiveDate::parse_from_str(&bill_date, "%Y-%m-%d").map_err(|_| "Invalid bill date".to_string());
            let Ok(bill_date) = bill_date else { set_error.set(bill_date.err()); return; };
            let due_date = NaiveDate::parse_from_str(&due_date, "%Y-%m-%d").map_err(|_| "Invalid due date".to_string());
            let Ok(due_date) = due_date else { set_error.set(due_date.err()); return; };
            if rows_val.is_empty() { set_error.set(Some("Add at least one line".into())); return; }
            let mut line_items: Vec<CreateBillLineItemRequest> = Vec::new();
            for (i, r) in rows_val.iter().enumerate() {
                let Some(acc) = r.expense_acc else { set_error.set(Some("Each line requires an expense account".into())); return; };
                let amount = r.amount.replace(",", "").parse::<Decimal>().map_err(|_| "Invalid amount".to_string());
                let Ok(amount) = amount else { set_error.set(amount.err()); return; };
                line_items.push(CreateBillLineItemRequest { line_number: (i+1) as i32, description: if r.desc.is_empty() { None } else { Some(r.desc.clone()) }, amount, expense_account_id: acc, billable: Some(r.billable), customer_id: r.customer });
            }
            let req = CreateBillRequest { bill_number: if bill_number.is_empty() { None } else { Some(bill_number) }, vendor_id, bill_date, due_date, memo: if memo.is_empty() { None } else { Some(memo) }, company_id: None, line_items };
            match bill_api::create_bill(&req).await {
                Ok(_) => { if let Some(t) = toaster { t.push("Bill created", ToastKind::Success); } navigate("/bills", Default::default()); }
                Err(e) => { if let Some(t) = toaster { t.push(e.clone(), ToastKind::Error); } set_error.set(Some(e)); }
            }
        }
    });

    view! {
        <div class="p-6 max-w-5xl">
            <h1 class="text-2xl font-semibold mb-4">"New Bill"</h1>
            {move || error.get().map(|e| view!{ <div class="text-red-600 mb-2">{e}</div> })}
            <form on:submit=move |ev| { ev.prevent_default(); on_submit.dispatch(()); }>
                <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
                    <div>
                        <label class="block mb-1">"Bill # (optional)"</label>
                        <input class="w-full border rounded px-3 py-2" type="text" prop:value=move || bill_number.get() on:input=move |e| set_bill_number.set(event_target_value(&e)) />
                    </div>
                    <div>
                        <label class="block mb-1">"Vendor"</label>
                        <Transition fallback=move || view!{ <div class="text-sm text-gray-500">"Loading..."</div> }>
                            {move || match vendors_res.get() {
                                Some(Ok(list)) => view!{
                                    <select class="w-full border rounded px-3 py-2" on:change=move |e| { let v = event_target_value(&e); if let Ok(id) = Uuid::parse_str(&v) { set_vendor_id.set(Some(id)); } }>
                                        <option value="">"Select vendor"</option>
                                        {list.into_iter().map(|c: Contact| view!{ <option value={c.id.to_string()}>{c.name}</option> }).collect_view()}
                                    </select>
                                }.into_view(),
                                Some(Err(e)) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                                None => view!{ <div/> }.into_view(),
                            }}
                        </Transition>
                    </div>
                    <div>
                        <label class="block mb-1">"Bill Date"</label>
                        <input class="w-full border rounded px-3 py-2" type="date" prop:value=move || bill_date_str.get() on:input=move |e| set_bill_date_str.set(event_target_value(&e)) />
                    </div>
                    <div>
                        <label class="block mb-1">"Due Date"</label>
                        <input class="w-full border rounded px-3 py-2" type="date" prop:value=move || due_date_str.get() on:input=move |e| set_due_date_str.set(event_target_value(&e)) />
                    </div>
                </div>

                <div class="mb-4">
                    <label class="block mb-1">"Memo (optional)"</label>
                    <textarea class="w-full border rounded px-3 py-2" rows=2 prop:value=move || memo.get() on:input=move |e| set_memo.set(event_target_value(&e)) />
                </div>

                <h2 class="text-lg font-semibold mb-2">"Line Items"</h2>
                <div class="overflow-auto">
                    <table class="w-full border-collapse bg-white dark:bg-gray-900 rounded shadow">
                        <thead>
                            <tr class="text-left border-b">
                                <th class="py-2 px-3 text-gray-600">"#"</th>
                                <th class="py-2 px-3 text-gray-600">"Description"</th>
                                <th class="py-2 px-3 text-gray-600">"Amount"</th>
                                <th class="py-2 px-3 text-gray-600">"Expense Account"</th>
                                <th class="py-2 px-3 text-gray-600">"Billable"</th>
                                <th class="py-2 px-3 text-gray-600">"Customer"</th>
                                <th class="py-2 px-3"></th>
                            </tr>
                        </thead>
                        <tbody>
                            {move || rows.get().into_iter().enumerate().map(|(i, r)| {
                                view!{
                                    <tr class="border-b">
                                        <td class="py-2 px-3">{i+1}</td>
                                        <td class="py-2 px-3"><input class="border rounded px-2 py-1 w-64" type="text" prop:value=r.desc.clone() on:input=move |e| set_rows.update(|v| v[i].desc = event_target_value(&e)) /></td>
                                        <td class="py-2 px-3">
                                            <input class="border rounded px-2 py-1 w-32" type="text" inputmode="decimal" placeholder="0.00"
                                                prop:value=r.amount.clone()
                                                on:input=move |e| set_rows.update(|v| v[i].amount = mask_money_input(&event_target_value(&e)))
                                            />
                                        </td>
                                        <td class="py-2 px-3">
                                            <Transition fallback=move || view!{<span class="text-gray-500">"Loading..."</span>}>
                                                {move || match expense_accounts_res.get() {
                                                    Some(Ok(list)) => view!{
                                                        <select class="border rounded px-2 py-1 w-64" on:change=move |e| {
                                                            let val = event_target_value(&e);
                                                            set_rows.update(|v| { if let Ok(id) = Uuid::parse_str(&val) { v[i].expense_acc = Some(id); } });
                                                        }>
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
                                            <input type="checkbox" prop:checked=r.billable on:change=move |_| set_rows.update(|v| v[i].billable = !v[i].billable) />
                                        </td>
                                        <td class="py-2 px-3">
                                            <Transition fallback=move || view!{<span class="text-gray-500">"Loading..."</span>}>
                                                {move || match customers_res.get() {
                                                    Some(Ok(list)) => view!{
                                                        <select class="border rounded px-2 py-1 w-64" on:change=move |e| {
                                                            let val = event_target_value(&e);
                                                            set_rows.update(|v| {
                                                                v[i].customer = if val.is_empty() { None } else { Uuid::parse_str(&val).ok() };
                                                            });
                                                        }>
                                                            <option value="">"None"</option>
                                                            {list.into_iter().map(|c: Contact| view!{ <option value={c.id.to_string()}>{c.name}</option> }).collect_view()}
                                                        </select>
                                                    }.into_view(),
                                                    Some(Err(e)) => view!{ <span class="text-red-600">{e}</span> }.into_view(),
                                                    None => view!{ <span/> }.into_view(),
                                                }}
                                            </Transition>
                                        </td>
                                        <td class="py-2 px-3 text-right"><button class="text-sm text-red-600 hover:underline" type="button" on:click=move |_| remove_row(i)>"Remove"</button></td>
                                    </tr>
                                }
                            }).collect_view()}
                        </tbody>
                        <tfoot>
                            <tr>
                                <td class="py-2 px-3" colspan="7">
                                    <button class="text-sm text-akowe-blue-600 hover:underline" type="button" on:click=add_row>"+ Add line"</button>
                                </td>
                            </tr>
                        </tfoot>
                    </table>
                </div>

                <div class="mt-4">"Total: "<span class="font-mono">{move || format_money(&total_amount())}</span></div>

                <div class="mt-6">
                    <button class="bg-blue-600 text-white px-4 py-2 rounded" type="submit">"Create Bill"</button>
                </div>
            </form>
        </div>
    }
}
