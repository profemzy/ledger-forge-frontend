use leptos::*;
use leptos_router::{use_params_map, A};
use uuid::Uuid;
use rust_decimal::Decimal;

use crate::api::{payments as pay_api, invoices as inv_api};
use crate::types::payments::Payment;
use crate::types::invoices::{Invoice, InvoiceStatus};
use crate::state::{ToastContext, ToastKind};
use crate::utils::format::format_money;

#[component]
pub fn PaymentDetail() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned());
    let toaster = use_context::<ToastContext>();

    // Load payment
    let payment_res = create_resource(
        move || id(),
        |id_opt| async move {
            let id_str = id_opt.ok_or_else(|| "Missing id".to_string())?;
            let id = Uuid::parse_str(&id_str).map_err(|_| "Invalid id".to_string())?;
            pay_api::get_payment(id).await
        }
    );

    // Load customer invoices when payment is available
    let invoices_res = create_resource(
        move || payment_res.get().and_then(|r| r.ok()).map(|p| p.customer_id),
        |cid_opt| async move {
            if let Some(cid) = cid_opt { inv_api::get_customer_invoices(cid).await } else { Ok(Vec::<Invoice>::new()) }
        },
    );

    // Applications state: invoice_id -> amount string
    let (apps, set_apps) = create_signal(std::collections::HashMap::<Uuid, String>::new());

    let total_applied = move || {
        apps.get().values().filter_map(|s| s.replace(",", "").parse::<Decimal>().ok()).fold(Decimal::ZERO, |a,b| a+b)
    };

    let apply_action = create_action(move |_: &()| {
        let payment = payment_res.get().and_then(|r| r.ok());
        let apps_map = apps.get();
        let toaster = toaster.clone();
        async move {
            if let Some(p) = payment {
                let mut list = Vec::new();
                for (inv_id, val) in apps_map.iter() {
                    let amt = val.replace(",", "").parse::<Decimal>().unwrap_or(Decimal::ZERO);
                    if amt > Decimal::ZERO { list.push(crate::types::payments::PaymentApplicationRequest { invoice_id: *inv_id, amount_applied: amt }); }
                }
                if list.is_empty() { if let Some(t) = toaster { t.push("Enter amounts to apply", ToastKind::Error); } return; }
                match pay_api::apply_payment(p.id, list).await {
                    Ok(_) => { if let Some(t) = toaster { t.push("Payment applied", ToastKind::Success); } }
                    Err(e) => { if let Some(t) = toaster { t.push(e, ToastKind::Error); } }
                }
            }
        }
    });

    view! {
        <div class="p-6 space-y-4">
            <Transition fallback=move || view!{ <div>"Loading payment..."</div> }>
                {move || match payment_res.get() {
                    Some(Ok(p)) => view!{ <PaymentView p=p/> }.into_view(),
                    Some(Err(e)) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                    None => view!{ <div/> }.into_view(),
                }}
            </Transition>

            <div class="bg-white rounded shadow p-4">
                <h2 class="text-lg font-semibold mb-2">"Apply Payment to Invoices"</h2>
                <Transition fallback=move || view!{ <div>"Loading invoices..."</div> }>
                    {move || match (payment_res.get(), invoices_res.get()) {
                        (Some(Ok(p)), Some(Ok(invoices))) => {
                            let open: Vec<Invoice> = invoices.into_iter().filter(|inv| inv.balance > Decimal::ZERO && inv.status != InvoiceStatus::Paid).collect();
                            if open.is_empty() {
                                view!{ <div class="text-gray-600">"No open invoices for this customer."</div> }.into_view()
                            } else {
                                view!{
                                    <table class="w-full border-collapse">
                                        <thead>
                                            <tr class="text-left border-b">
                                                <th class="py-2 px-3">"Invoice #"</th>
                                                <th class="py-2 px-3">"Date"</th>
                                                <th class="py-2 px-3">"Due"</th>
                                                <th class="py-2 px-3">"Balance"</th>
                                                <th class="py-2 px-3">"Apply"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {open.into_iter().map(|inv| view!{
                                                <tr class="border-b">
                                                    <td class="py-2 px-3"><A class="text-akowe-blue-600 hover:underline" href=format!("/invoices/{}", inv.id)>{inv.invoice_number.clone()}</A></td>
                                                    <td class="py-2 px-3">{inv.invoice_date.to_string()}</td>
                                                    <td class="py-2 px-3">{inv.due_date.to_string()}</td>
                                                            <td class="py-2 px-3">{format_money(&inv.balance)}</td>
                                                    <td class="py-2 px-3">
                                                        <input class="border rounded px-2 py-1 w-32" type="text" inputmode="decimal" placeholder="0.00"
                                                            on:input=move |e| {
                                                                let v = event_target_value(&e);
                                                                set_apps.update(|m| { m.insert(inv.id, v); });
                                                            }
                                                        />
                                                    </td>
                                                </tr>
                                            }).collect_view()}
                                        </tbody>
                                    </table>
                                    <div class="mt-4 flex gap-6 items-center">
                                        <div>"Unapplied Available: "<span class="font-mono">{format_money(&p.unapplied_amount.unwrap_or_default())}</span></div>
                                        <div>"Apply Now: "<span class="font-mono">{move || format_money(&total_applied())}</span></div>
                                    </div>
                                    <div class="mt-4">
                                        <button class="bg-blue-600 text-white px-4 py-2 rounded" on:click=move |_| apply_action.dispatch(())>"Apply Payment"</button>
                                    </div>
                                }.into_view()
                            }
                        }
                        (Some(Err(e)), _) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                        _ => view!{ <div/> }.into_view(),
                    }}
                </Transition>
            </div>
        </div>
    }
}

#[component]
fn PaymentView(p: Payment) -> impl IntoView {
    view! {
        <div class="bg-white rounded shadow p-4">
            <div class="flex items-center justify-between">
                <div>
                    <div class="text-xl font-semibold">{"Payment "}{p.payment_number.clone().unwrap_or_else(|| p.id.to_string())}</div>
                    <div class="text-gray-600">{"Date: "}{p.payment_date.to_string()} {" • Method: "}{p.payment_method.clone()} {" • Ref: "}{p.reference_number.clone().unwrap_or_else(|| "—".into())}</div>
                    <div class="mt-1">
                        <span class="text-xs px-2 py-1 rounded bg-blue-100 text-blue-700 border">{"Amount: "}{format_money(&p.amount)}</span>
                        <span class="ml-2 text-xs px-2 py-1 rounded bg-yellow-100 text-yellow-700 border">{"Unapplied: "}{format_money(&p.unapplied_amount.unwrap_or_default())}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
