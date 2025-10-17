use leptos::*;
use leptos_router::{use_params_map, A};
use uuid::Uuid;

use crate::api::invoices as api;
use crate::types::invoices::{InvoiceWithLineItems, InvoiceStatus};
use crate::types::payments::Payment;
use crate::state::{ToastContext, ToastKind};
use crate::utils::format::format_money;

#[component]
pub fn InvoiceDetail() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned());
    let toaster = use_context::<ToastContext>();

    let inv_res = create_resource(
        move || id(),
        |id_opt| async move {
            let id_str = id_opt.ok_or_else(|| "Missing id".to_string())?;
            let id = Uuid::parse_str(&id_str).map_err(|_| "Invalid id".to_string())?;
            api::get_invoice(id).await
        }
    );

    let payments_res = create_resource(
        move || id(),
        |id_opt| async move {
            let id_str = id_opt.ok_or_else(|| "Missing id".to_string())?;
            let id = Uuid::parse_str(&id_str).map_err(|_| "Invalid id".to_string())?;
            api::get_invoice_payments(id).await
        }
    );

    // Status actions
    let set_status = |status: InvoiceStatus| {
        let inv_res = inv_res.clone();
        let toaster = toaster.clone();
        create_action(move |_: &()| {
            let inv = inv_res.get().and_then(|r| r.ok()).map(|i| i.invoice.id);
            let toaster = toaster.clone();
            let status_value = status.clone();
            async move {
                if let Some(id) = inv {
                    match api::update_invoice_status(id, status_value.clone()).await {
                        Ok(_) => { if let Some(t) = toaster { t.push(format!("Invoice marked {:?}", status_value).to_lowercase(), ToastKind::Success); } inv_res.refetch(); }
                        Err(e) => { if let Some(t) = toaster { t.push(e, ToastKind::Error); } }
                    }
                }
            }
        })
    };
    let mark_sent = set_status(InvoiceStatus::Sent);
    let mark_paid = set_status(InvoiceStatus::Paid);
    let mark_overdue = set_status(InvoiceStatus::Overdue);
    let mark_void = set_status(InvoiceStatus::Void);

    view! {
        <div class="p-6 space-y-4">
            <Transition fallback=move || view!{ <div>"Loading invoice..."</div> }>
                {move || match inv_res.get() {
                    Some(Ok(inv)) => view!{ <InvoiceView inv=inv mark_sent=mark_sent.clone() mark_paid=mark_paid.clone() mark_overdue=mark_overdue.clone() mark_void=mark_void.clone()/> }.into_view(),
                    Some(Err(e)) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                    None => view!{ <div/> }.into_view(),
                }}
            </Transition>

            <div class="bg-white rounded shadow p-4">
                <h2 class="text-lg font-semibold mb-2">"Payments"</h2>
                <Transition fallback=move || view!{ <div>"Loading payments..."</div> }>
                    {move || match payments_res.get() {
                        Some(Ok(list)) => view!{ <PaymentsTable items=list/> }.into_view(),
                        Some(Err(e)) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                        None => view!{ <div/> }.into_view(),
                    }}
                </Transition>
            </div>
        </div>
    }
}

#[component]
fn InvoiceView(inv: InvoiceWithLineItems, mark_sent: Action<(), ()>, mark_paid: Action<(), ()>, mark_overdue: Action<(), ()>, mark_void: Action<(), ()>) -> impl IntoView {
    view! {
        <div class="space-y-4">
            <div class="text-sm text-gray-600"><A class="text-akowe-blue-600 hover:underline" href="/invoices">"Invoices"</A> " / " {inv.invoice.invoice_number.clone()}</div>
            <div class="bg-white rounded shadow p-4">
                <div class="flex items-center justify-between">
                    <div>
                        <div class="text-xl font-semibold">{"Invoice "}{inv.invoice.invoice_number.clone()}</div>
                        <div class="text-gray-600">{"Date: "}{inv.invoice.invoice_date.to_string()} {" • Due: "}{inv.invoice.due_date.to_string()}</div>
                        <div class="mt-1">
                            <span class="text-xs px-2 py-1 rounded bg-gray-100 text-gray-700 border">{"Status: "}{format!("{:?}", inv.invoice.status).to_lowercase()}</span>
                            <span class="ml-2 text-xs px-2 py-1 rounded bg-blue-100 text-blue-700 border">{"Total: "}{format_money(&inv.invoice.total_amount)}</span>
                            <span class="ml-2 text-xs px-2 py-1 rounded bg-yellow-100 text-yellow-700 border">{"Balance: "}{format_money(&inv.invoice.balance)}</span>
                        </div>
                    </div>
                    <div class="space-x-2">
                        <button class="bg-gray-600 text-white px-3 py-2 rounded" on:click=move |_| mark_sent.dispatch(())>"Mark Sent"</button>
                        <button class="bg-green-600 text-white px-3 py-2 rounded" on:click=move |_| mark_paid.dispatch(())>"Mark Paid"</button>
                        <button class="bg-red-600 text-white px-3 py-2 rounded" on:click=move |_| mark_void.dispatch(())>"Void"</button>
                        <button class="bg-yellow-600 text-white px-3 py-2 rounded" on:click=move |_| mark_overdue.dispatch(())>"Mark Overdue"</button>
                    </div>
                </div>
            </div>

            <div class="bg-white rounded shadow">
                <table class="w-full border-collapse">
                    <thead>
                        <tr class="text-left border-b">
                            <th class="py-2 px-3 text-gray-600">"Line"</th>
                            <th class="py-2 px-3 text-gray-600">"Description"</th>
                            <th class="py-2 px-3 text-gray-600">"Qty"</th>
                            <th class="py-2 px-3 text-gray-600">"Unit Price"</th>
                            <th class="py-2 px-3 text-gray-600">"Amount"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {inv.line_items.into_iter().map(|li| view!{
                            <tr class="border-b">
                                <td class="py-2 px-3">{li.line_number}</td>
                                <td class="py-2 px-3">{li.item_description.clone()}</td>
                                <td class="py-2 px-3">{li.quantity.to_string()}</td>
                                <td class="py-2 px-3">{format_money(&li.unit_price)}</td>
                                <td class="py-2 px-3">{format_money(&li.amount)}</td>
                            </tr>
                        }).collect_view()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[component]
fn PaymentsTable(items: Vec<Payment>) -> impl IntoView {
    view! {
        <table class="w-full border-collapse">
            <thead>
                <tr class="text-left border-b">
                    <th class="py-2 px-3 text-gray-600">"Date"</th>
                    <th class="py-2 px-3 text-gray-600">"Number"</th>
                    <th class="py-2 px-3 text-gray-600">"Amount"</th>
                    <th class="py-2 px-3 text-gray-600">"Unapplied"</th>
                </tr>
            </thead>
            <tbody>
                {items.into_iter().map(|p| view!{
                    <tr class="border-b">
                        <td class="py-2 px-3">{p.payment_date.to_string()}</td>
                        <td class="py-2 px-3 font-mono text-sm"><A class="text-akowe-blue-600 hover:underline" href=format!("/payments/{}", p.id)>{p.payment_number.unwrap_or_else(|| "—".into())}</A></td>
                        <td class="py-2 px-3">{format_money(&p.amount)}</td>
                        <td class="py-2 px-3">{format_money(&p.unapplied_amount.unwrap_or_default())}</td>
                    </tr>
                }).collect_view()}
            </tbody>
        </table>
    }
}
