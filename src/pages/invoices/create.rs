use chrono::NaiveDate;
use leptos::*;
use leptos_router::use_navigate;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::api::{invoices as inv_api, contacts as contacts_api, accounts as accounts_api};
use crate::types::invoices::{CreateInvoiceRequest, CreateInvoiceLineItemRequest};
use crate::types::contacts::Contact;
use crate::types::accounts::{Account, AccountType};
use crate::state::{ToastContext, ToastKind};

#[component]
pub fn InvoiceCreate() -> impl IntoView {
    // Header fields
    let (invoice_number, set_invoice_number) = create_signal(String::new());
    let (customer_id, set_customer_id) = create_signal(None::<Uuid>);
    let (invoice_date_str, set_invoice_date_str) = create_signal(String::new());
    let (due_date_str, set_due_date_str) = create_signal(String::new());
    let (ship_date_str, set_ship_date_str) = create_signal(String::new());
    let (memo, set_memo) = create_signal(String::new());
    let (billing_address, set_billing_address) = create_signal(String::new());
    let (shipping_address, set_shipping_address) = create_signal(String::new());
    let (error, set_error) = create_signal(None::<String>);
    let navigate = use_navigate();
    let toaster = use_context::<ToastContext>();

    // Load customers and revenue accounts
    let customers_res = create_resource(|| (), |_| async move { contacts_api::list_customers().await });
    let revenue_accounts_res = create_resource(|| (), |_| async move { accounts_api::list_accounts(Some(AccountType::Revenue), None, Some(false)).await });

    // Line items state
    #[derive(Clone)]
    struct Row { desc: String, qty: String, unit: String, disc: String, rev_acc: Option<Uuid> }
    let (rows, set_rows) = create_signal::<Vec<Row>>(vec![Row { desc: String::new(), qty: String::new(), unit: String::new(), disc: String::new(), rev_acc: None }]);

    let add_row = move |_| set_rows.update(|v| v.push(Row { desc: String::new(), qty: String::new(), unit: String::new(), disc: String::new(), rev_acc: None }));
    let remove_row = move |i: usize| set_rows.update(|v| { if v.len() > 1 { v.remove(i); } });

    // Helpers to compute line amount
    let calc_line_amount = |r: &Row| -> Option<Decimal> {
        let qty = r.qty.replace(",", "").parse::<Decimal>().ok()?;
        let unit = r.unit.replace(",", "").parse::<Decimal>().ok()?;
        if qty <= Decimal::ZERO || unit <= Decimal::ZERO { return None; }
        let mut amount = qty * unit;
        if let Ok(p) = r.disc.replace("%", "").replace(",", "").parse::<Decimal>() {
            if p > Decimal::ZERO { amount -= amount * (p / Decimal::new(100, 0)); }
        }
        Some(amount)
    };
    let total_amount = move || rows.get().iter().filter_map(calc_line_amount).fold(Decimal::ZERO, |a, b| a + b);

    let on_submit = create_action(move |_: &()| {
        let invoice_number = invoice_number.get();
        let customer_id = customer_id.get();
        let inv_date = invoice_date_str.get();
        let due_date = due_date_str.get();
        let ship_date = ship_date_str.get();
        let memo = memo.get();
        let bill_addr = billing_address.get();
        let ship_addr = shipping_address.get();
        let rows_val = rows.get();
        let navigate = navigate.clone();
        let toaster = toaster.clone();
        async move {
            set_error.set(None);

            if invoice_number.trim().is_empty() { set_error.set(Some("Invoice number is required".into())); return; }
            let Some(customer_id) = customer_id else { set_error.set(Some("Customer is required".into())); return; };
            let inv_date = NaiveDate::parse_from_str(&inv_date, "%Y-%m-%d").map_err(|_| "Invalid invoice date".to_string());
            let Ok(inv_date) = inv_date else { set_error.set(inv_date.err()); return; };
            let due_date = NaiveDate::parse_from_str(&due_date, "%Y-%m-%d").map_err(|_| "Invalid due date".to_string());
            let Ok(due_date) = due_date else { set_error.set(due_date.err()); return; };
            let ship_date = if ship_date.trim().is_empty() { None } else { NaiveDate::parse_from_str(&ship_date, "%Y-%m-%d").ok() };

            if rows_val.is_empty() { set_error.set(Some("Add at least one line".into())); return; }
            let mut line_items: Vec<CreateInvoiceLineItemRequest> = Vec::new();
            for (i, r) in rows_val.iter().enumerate() {
                let Some(rev) = r.rev_acc else { set_error.set(Some("Each line requires a revenue account".into())); return; };
                let qty = r.qty.replace(",", "").parse::<Decimal>().map_err(|_| "Invalid quantity".to_string());
                let Ok(qty) = qty else { set_error.set(qty.err()); return; };
                let unit = r.unit.replace(",", "").parse::<Decimal>().map_err(|_| "Invalid unit price".to_string());
                let Ok(unit) = unit else { set_error.set(unit.err()); return; };
                let disc = if r.disc.trim().is_empty() { None } else { r.disc.replace("%", "").replace(",", "").parse::<Decimal>().ok() };
                // amount will be calculated by server, we send computed discount percent only
                line_items.push(CreateInvoiceLineItemRequest {
                    line_number: (i as i32) + 1,
                    item_description: r.desc.clone(),
                    quantity: qty,
                    unit_price: unit,
                    discount_percent: disc,
                    tax_code: None,
                    revenue_account_id: rev,
                });
            }

            let req = CreateInvoiceRequest {
                invoice_number,
                customer_id,
                invoice_date: inv_date,
                due_date,
                ship_date,
                customer_memo: if memo.is_empty() { None } else { Some(memo) },
                billing_address: if bill_addr.is_empty() { None } else { Some(bill_addr) },
                shipping_address: if ship_addr.is_empty() { None } else { Some(ship_addr) },
                company_id: None,
                line_items,
            };

            match inv_api::create_invoice(&req).await {
                Ok(inv) => {
                    if let Some(t) = toaster { t.push("Invoice created", ToastKind::Success); }
                    navigate(&format!("/invoices/{}", inv.invoice.id), Default::default());
                }
                Err(e) => set_error.set(Some(e)),
            }
        }
    });

    view! {
        <div class="p-6 max-w-5xl">
            <h1 class="text-2xl font-semibold mb-4">"New Invoice"</h1>
            {move || error.get().map(|e| view!{ <div class="text-red-600 mb-2">{e}</div> })}

            <form on:submit=move |ev| { ev.prevent_default(); on_submit.dispatch(()); }>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
                    <div>
                        <label class="block mb-1">"Invoice #"</label>
                        <input class="w-full border rounded px-3 py-2" type="text"
                            prop:value=move || invoice_number.get()
                            on:input=move |e| set_invoice_number.set(event_target_value(&e))
                        />
                    </div>
                    <div>
                        <label class="block mb-1">"Customer"</label>
                        <Transition fallback=move || view!{ <div class="text-sm text-gray-500">"Loading..."</div> }>
                            {move || match customers_res.get() {
                                Some(Ok(list)) => view!{
                                    <select class="w-full border rounded px-3 py-2"
                                        on:change=move |e| {
                                            let v = event_target_value(&e);
                                            if let Ok(id) = Uuid::parse_str(&v) { set_customer_id.set(Some(id)); }
                                        }
                                    >
                                        <option value="">"Select customer"</option>
                                        {list.into_iter().map(|c: Contact| view!{ <option value={c.id.to_string()}>{c.name}</option> }).collect_view()}
                                    </select>
                                }.into_view(),
                                Some(Err(e)) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                                None => view!{ <div/> }.into_view(),
                            }}
                        </Transition>
                    </div>
                    <div>
                        <label class="block mb-1">"Invoice Date"</label>
                        <input class="w-full border rounded px-3 py-2" type="date"
                            prop:value=move || invoice_date_str.get()
                            on:input=move |e| set_invoice_date_str.set(event_target_value(&e))
                        />
                    </div>
                    <div>
                        <label class="block mb-1">"Due Date"</label>
                        <input class="w-full border rounded px-3 py-2" type="date"
                            prop:value=move || due_date_str.get()
                            on:input=move |e| set_due_date_str.set(event_target_value(&e))
                        />
                    </div>
                    <div>
                        <label class="block mb-1">"Ship Date"</label>
                        <input class="w-full border rounded px-3 py-2" type="date"
                            prop:value=move || ship_date_str.get()
                            on:input=move |e| set_ship_date_str.set(event_target_value(&e))
                        />
                    </div>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                    <div>
                        <label class="block mb-1">"Customer Memo"</label>
                        <textarea class="w-full border rounded px-3 py-2" rows=3
                            prop:value=move || memo.get()
                            on:input=move |e| set_memo.set(event_target_value(&e))
                        />
                    </div>
                    <div class="grid grid-cols-1 gap-4">
                        <div>
                            <label class="block mb-1">"Billing Address"</label>
                            <textarea class="w-full border rounded px-3 py-2" rows=3
                                prop:value=move || billing_address.get()
                                on:input=move |e| set_billing_address.set(event_target_value(&e))
                            />
                        </div>
                        <div>
                            <label class="block mb-1">"Shipping Address"</label>
                            <textarea class="w-full border rounded px-3 py-2" rows=3
                                prop:value=move || shipping_address.get()
                                on:input=move |e| set_shipping_address.set(event_target_value(&e))
                            />
                        </div>
                    </div>
                </div>

                <h2 class="text-lg font-semibold mb-2">"Line Items"</h2>
                <div class="overflow-auto">
                    <table class="w-full border-collapse bg-white rounded shadow">
                        <thead>
                            <tr class="text-left border-b">
                                <th class="py-2 px-3 text-gray-600">"#"</th>
                                <th class="py-2 px-3 text-gray-600">"Description"</th>
                                <th class="py-2 px-3 text-gray-600">"Qty"</th>
                                <th class="py-2 px-3 text-gray-600">"Unit Price"</th>
                                <th class="py-2 px-3 text-gray-600">"Discount %"</th>
                                <th class="py-2 px-3 text-gray-600">"Revenue Account"</th>
                                <th class="py-2 px-3 text-gray-600">"Amount"</th>
                                <th class="py-2 px-3"></th>
                            </tr>
                        </thead>
                        <tbody>
                            {move || rows.get().into_iter().enumerate().map(|(i, r)| {
                                let amount = calc_line_amount(&r).unwrap_or(Decimal::ZERO);
                                view!{
                                    <tr class="border-b">
                                        <td class="py-2 px-3">{i+1}</td>
                                        <td class="py-2 px-3">
                                            <input class="border rounded px-2 py-1 w-64" type="text"
                                                prop:value=r.desc.clone()
                                                on:input=move |e| set_rows.update(|v| v[i].desc = event_target_value(&e))
                                            />
                                        </td>
                                        <td class="py-2 px-3">
                                            <input class="border rounded px-2 py-1 w-24" type="text" inputmode="decimal" placeholder="0.00"
                                                prop:value=r.qty.clone()
                                                on:input=move |e| set_rows.update(|v| v[i].qty = crate::utils::format::mask_money_input(&event_target_value(&e)))
                                            />
                                        </td>
                                        <td class="py-2 px-3">
                                            <input class="border rounded px-2 py-1 w-24" type="text" inputmode="decimal" placeholder="0.00"
                                                prop:value=r.unit.clone()
                                                on:input=move |e| set_rows.update(|v| v[i].unit = crate::utils::format::mask_money_input(&event_target_value(&e)))
                                            />
                                        </td>
                                        <td class="py-2 px-3">
                                            <input class="border rounded px-2 py-1 w-20" type="text" inputmode="decimal" placeholder="0"
                                                prop:value=r.disc.clone()
                                                on:input=move |e| set_rows.update(|v| v[i].disc = crate::utils::format::mask_money_input(&event_target_value(&e)))
                                            />
                                        </td>
                                        <td class="py-2 px-3">
                                            <Transition fallback=move || view!{ <span class="text-gray-500">"Loading..."</span> }>
                                                {move || match revenue_accounts_res.get() {
                                                    Some(Ok(list)) => view!{
                                                        <select class="border rounded px-2 py-1 w-64"
                                                            on:change=move |e| {
                                                                let v = event_target_value(&e);
                                                                set_rows.update(|vrows| { if let Ok(id) = Uuid::parse_str(&v) { vrows[i].rev_acc = Some(id); } });
                                                            }
                                                        >
                                                            <option value="">"Select revenue account"</option>
                                                            {list.into_iter().map(|a: Account| view!{ <option value={a.id.to_string()}>{format!("{} - {}", a.code, a.name)}</option> }).collect_view()}
                                                        </select>
                                                    }.into_view(),
                                                    Some(Err(e)) => view!{ <span class="text-red-600">{e}</span> }.into_view(),
                                                    None => view!{ <span/> }.into_view(),
                                                }}
                                            </Transition>
                                        </td>
                                        <td class="py-2 px-3 font-mono">{amount.to_string()}</td>
                                        <td class="py-2 px-3 text-right">
                                            <button class="text-sm text-red-600 hover:underline" type="button" on:click=move |_| remove_row(i)>"Remove"</button>
                                        </td>
                                    </tr>
                                }
                            }).collect_view()}
                        </tbody>
                        <tfoot>
                            <tr>
                                <td class="py-2 px-3" colspan="8">
                                    <button class="text-sm text-akowe-blue-600 hover:underline" type="button" on:click=add_row>"+ Add line"</button>
                                </td>
                            </tr>
                        </tfoot>
                    </table>
                </div>

                <div class="mt-4 flex gap-6 items-center">
                    <div>"Total: "<span class="font-mono">{move || total_amount().to_string()}</span></div>
                </div>

                <div class="mt-6">
                    <button class="bg-blue-600 text-white px-4 py-2 rounded" type="submit">"Create Invoice"</button>
                </div>
            </form>
        </div>
    }
}
