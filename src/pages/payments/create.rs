use chrono::NaiveDate;
use leptos::*;
use leptos_router::use_navigate;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::api::{payments as pay_api, contacts as contacts_api, invoices as inv_api, accounts as accounts_api};
use crate::types::payments::{CreatePaymentRequest, PaymentApplicationRequest};
use crate::components::ui::MoneyInput;
use crate::types::contacts::Contact;
use crate::types::invoices::Invoice;
use crate::types::accounts::{Account, AccountType};
use crate::state::{ToastContext, ToastKind};
use crate::utils::format::{format_money, mask_money_input};

#[component]
pub fn PaymentCreate() -> impl IntoView {
    let (payment_number, set_payment_number) = create_signal(String::new());
    let (customer_id, set_customer_id) = create_signal(None::<Uuid>);
    let (payment_date_str, set_payment_date_str) = create_signal(String::new());
    let (amount_str, set_amount_str) = create_signal(String::new());
    let (method, set_method) = create_signal(String::new());
    let (reference, set_reference) = create_signal(String::new());
    let (deposit_account, set_deposit_account) = create_signal(None::<Uuid>);
    let (memo, set_memo) = create_signal(String::new());
    let (error, set_error) = create_signal(None::<String>);
    let navigate = use_navigate();
    let toaster = use_context::<ToastContext>();

    // Customers, deposit accounts (Assets), and invoices for selected customer
    let customers_res = create_resource(|| (), |_| async move { contacts_api::list_customers().await });
    let deposit_accounts_res = create_resource(|| (), |_| async move { accounts_api::list_accounts(Some(AccountType::Asset), None, Some(false)).await });
    let invoices_res = create_resource(move || customer_id.get(), |cid| async move {
        if let Some(c) = cid { inv_api::get_customer_invoices(c).await } else { Ok(Vec::<Invoice>::new()) }
    });

    // Applications state: map invoice_id -> amount string
    use std::collections::HashMap;
    let (apps, set_apps) = create_signal::<HashMap<Uuid, String>>(HashMap::new());

    let total_applied = move || {
        apps.get().values().filter_map(|s| s.replace(",", "").parse::<Decimal>().ok()).fold(Decimal::ZERO, |a,b| a+b)
    };
    let entered_amount = move || amount_str.get().replace(",", "").parse::<Decimal>().unwrap_or(Decimal::ZERO);
    let unapplied = move || entered_amount() - total_applied();

    let on_submit = create_action(move |_: &()| {
        let pnum = payment_number.get();
        let cid = customer_id.get();
        let pdate = payment_date_str.get();
        let amt = amount_str.get();
        let method = method.get();
        let reference = reference.get();
        let deposit = deposit_account.get();
        let memo = memo.get();
        let apps_map = apps.get();
        let navigate = navigate.clone();
        let toaster = toaster.clone();
        async move {
            set_error.set(None);
            let Some(customer_id) = cid else { set_error.set(Some("Customer is required".into())); return; };
            let payment_date = NaiveDate::parse_from_str(&pdate, "%Y-%m-%d").map_err(|_| "Invalid date".to_string());
            let Ok(payment_date) = payment_date else { set_error.set(payment_date.err()); return; };
            if method.trim().is_empty() { set_error.set(Some("Payment method required".into())); return; }
            let amount = amt.replace(",", "").parse::<Decimal>().map_err(|_| "Invalid amount".to_string());
            let Ok(amount) = amount else { set_error.set(amount.err()); return; };
            if amount <= Decimal::ZERO { set_error.set(Some("Amount must be > 0".into())); return; }
            if total_applied() > amount { set_error.set(Some("Applied amount exceeds payment amount".into())); return; }

            // Build applications
            let mut applications: Vec<PaymentApplicationRequest> = Vec::new();
            for (inv_id, val) in apps_map.into_iter() {
                let v = val.replace(",", "").parse::<Decimal>().unwrap_or(Decimal::ZERO);
                if v > Decimal::ZERO { applications.push(PaymentApplicationRequest { invoice_id: inv_id, amount_applied: v }); }
            }

            let req = CreatePaymentRequest {
                payment_number: if pnum.is_empty() { None } else { Some(pnum) },
                customer_id,
                payment_date,
                amount,
                payment_method: method,
                reference_number: if reference.is_empty() { None } else { Some(reference) },
                deposit_to_account_id: deposit,
                memo: if memo.is_empty() { None } else { Some(memo) },
                company_id: None,
                applications,
            };

            match pay_api::create_payment(&req).await {
                Ok(_) => { if let Some(t) = toaster { t.push("Payment recorded", ToastKind::Success); } navigate("/payments", Default::default()); }
                Err(e) => { if let Some(t) = toaster { t.push(e.clone(), ToastKind::Error); } set_error.set(Some(e)); }
            }
        }
    });

    view! {
        <div class="p-6 max-w-5xl">
            <h1 class="text-2xl font-semibold mb-4">"New Payment"</h1>
            {move || error.get().map(|e| view!{ <div class="text-red-600 mb-2">{e}</div> })}
            <form on:submit=move |ev| { ev.prevent_default(); on_submit.dispatch(()); }>
                <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
                    <div>
                        <label class="block mb-1">"Payment # (optional)"</label>
                        <input class="w-full border rounded px-3 py-2" type="text"
                            prop:value=move || payment_number.get()
                            on:input=move |e| set_payment_number.set(event_target_value(&e))
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
                                            if let Ok(id) = Uuid::parse_str(&v) { set_customer_id.set(Some(id)); set_apps.set(std::collections::HashMap::new()); }
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
                        <label class="block mb-1">"Payment Date"</label>
                        <input class="w-full border rounded px-3 py-2" type="date"
                            prop:value=move || payment_date_str.get()
                            on:input=move |e| set_payment_date_str.set(event_target_value(&e))
                        />
                    </div>
                    <div>
                        <label class="block mb-1">"Amount"</label>
                        <MoneyInput value=amount_str set_value=set_amount_str placeholder="0.00" class="w-full border rounded px-3 py-2" />
                    </div>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
                    <div>
                        <label class="block mb-1">"Payment Method"</label>
                        <input class="w-full border rounded px-3 py-2" type="text" placeholder="Check / Cash / Card"
                            prop:value=move || method.get()
                            on:input=move |e| set_method.set(event_target_value(&e))
                        />
                    </div>
                    <div>
                        <label class="block mb-1">"Reference #"</label>
                        <input class="w-full border rounded px-3 py-2" type="text"
                            prop:value=move || reference.get()
                            on:input=move |e| set_reference.set(event_target_value(&e))
                        />
                    </div>
                    <div>
                        <label class="block mb-1">"Deposit To Account"</label>
                        <Transition fallback=move || view!{ <div class="text-sm text-gray-500">"Loading..."</div> }>
                            {move || match deposit_accounts_res.get() {
                                Some(Ok(list)) => view!{
                                    <select class="w-full border rounded px-3 py-2"
                                        on:change=move |e| {
                                            let v = event_target_value(&e);
                                            if v.is_empty() { set_deposit_account.set(None); } else if let Ok(id) = Uuid::parse_str(&v) { set_deposit_account.set(Some(id)); }
                                        }
                                    >
                                        <option value="">"Default"</option>
                                        {list.into_iter().map(|a: Account| view!{ <option value={a.id.to_string()}>{format!("{} - {}", a.code, a.name)}</option> }).collect_view()}
                                    </select>
                                }.into_view(),
                                Some(Err(e)) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                                None => view!{ <div/> }.into_view(),
                            }}
                        </Transition>
                    </div>
                </div>

                <div class="mb-4">
                    <label class="block mb-1">"Memo (optional)"</label>
                    <textarea class="w-full border rounded px-3 py-2" rows=2
                        prop:value=move || memo.get()
                        on:input=move |e| set_memo.set(event_target_value(&e))
                    />
                </div>

                <h2 class="text-lg font-semibold mb-2">"Apply to Invoices"</h2>
                <div class="overflow-auto">
                    <Transition fallback=move || view!{ <div>"Loading invoices..."</div> }>
                        {move || match invoices_res.get() {
                            Some(Ok(list)) => {
                                let open_invoices: Vec<Invoice> = list.into_iter().filter(|inv| inv.balance > Decimal::ZERO && inv.status != crate::types::invoices::InvoiceStatus::Paid).collect();
                                if open_invoices.is_empty() {
                                    view!{ <div class="text-gray-600">"No open invoices for this customer."</div> }.into_view()
                                } else {
                                    view!{
                                        <table class="w-full border-collapse bg-white rounded shadow">
                                            <thead>
                                                <tr class="text-left border-b">
                                                    <th class="py-2 px-3 text-gray-600">"Invoice #"</th>
                                                    <th class="py-2 px-3 text-gray-600">"Date"</th>
                                                    <th class="py-2 px-3 text-gray-600">"Due"</th>
                                                    <th class="py-2 px-3 text-gray-600">"Balance"</th>
                                                    <th class="py-2 px-3 text-gray-600">"Apply"</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {open_invoices.into_iter().map(|inv| {
                                                    view!{
                                                        <tr class="border-b">
                                                            <td class="py-2 px-3">{inv.invoice_number.clone()}</td>
                                                            <td class="py-2 px-3">{inv.invoice_date.to_string()}</td>
                                                            <td class="py-2 px-3">{inv.due_date.to_string()}</td>
                                                            <td class="py-2 px-3">{format_money(&inv.balance)}</td>
                                                            <td class="py-2 px-3">
                                                                <input class="border rounded px-2 py-1 w-32" type="text" inputmode="decimal" placeholder="0.00"
                                                                    on:input=move |e| {
                                                                        let v = mask_money_input(&event_target_value(&e));
                                                                        set_apps.update(|m| { m.insert(inv.id, v); });
                                                                    }
                                                                />
                                                            </td>
                                                        </tr>
                                                    }
                                                }).collect_view()}
                                            </tbody>
                                        </table>
                                    }.into_view()
                                }
                            }
                            Some(Err(e)) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                            None => view!{ <div/> }.into_view(),
                        }}
                    </Transition>
                </div>

                <div class="mt-4 flex gap-6 items-center">
                    <div>"Total Applied: "<span class="font-mono">{move || format_money(&total_applied())}</span></div>
                    <div>"Unapplied: "<span class="font-mono">{move || format_money(&unapplied())}</span></div>
                </div>

                <div class="mt-6">
                    <button class="bg-blue-600 text-white px-4 py-2 rounded" type="submit">"Record Payment"</button>
                </div>
            </form>
        </div>
    }
}
