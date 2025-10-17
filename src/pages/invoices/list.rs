use leptos::*;
use leptos_router::{A, use_query_map};
use uuid::Uuid;

use crate::api::invoices as api;
use crate::components::ui::{ButtonLink, Table};
use crate::types::invoices::{Invoice, InvoiceStatus};
use crate::utils::format::format_money;
use crate::state::{ToastContext, ToastKind};

#[component]
pub fn InvoicesList() -> impl IntoView {
    let (status, set_status) = create_signal(None::<InvoiceStatus>);
    let (customer_id_str, set_customer_id_str) = create_signal(String::new());
    let query_map = use_query_map();

    let invoices = create_resource(
        move || (status.get(), customer_id_str.get()),
        |(s, cid)| async move {
            let customer_id = if cid.trim().is_empty() { None } else { Uuid::parse_str(&cid).ok() };
            api::list_invoices(customer_id, s, Some(100), None).await
        },
    );

    // Initialize filters from query params (?status=overdue&customer=<uuid>)
    create_effect(move |_| {
        let qs_status = query_map.with(|q| q.get("status").cloned());
        if let Some(s) = qs_status {
            let st = match s.to_lowercase().as_str() {
                "draft" => Some(InvoiceStatus::Draft),
                "sent" => Some(InvoiceStatus::Sent),
                "paid" => Some(InvoiceStatus::Paid),
                "partial" => Some(InvoiceStatus::Partial),
                "overdue" => Some(InvoiceStatus::Overdue),
                "void" => Some(InvoiceStatus::Void),
                _ => None,
            };
            if st.is_some() { set_status.set(st); }
        }
        let qs_customer = query_map.with(|q| q.get("customer").cloned());
        if let Some(cid) = qs_customer { set_customer_id_str.set(cid); }
    });

    view! {
        <div class="p-6">
            <div class="flex items-center justify-between mb-4">
                <h1 class="text-2xl font-semibold">"Invoices"</h1>
                <ButtonLink href="/invoices/new" variant="primary">{"New Invoice"}</ButtonLink>
            </div>

            <div class="flex gap-4 mb-4 items-center">
                <crate::components::ui::TextInput value=customer_id_str set_value=set_customer_id_str placeholder="Filter by Customer ID (UUID)" class="border rounded px-3 py-2 w-72" inputmode="text" />
                <select class="border rounded px-2 py-1"
                    on:change=move |e| {
                        let v = event_target_value(&e);
                        let s = match v.as_str() {
                            "draft" => Some(InvoiceStatus::Draft),
                            "sent" => Some(InvoiceStatus::Sent),
                            "paid" => Some(InvoiceStatus::Paid),
                            "partial" => Some(InvoiceStatus::Partial),
                            "overdue" => Some(InvoiceStatus::Overdue),
                            "void" => Some(InvoiceStatus::Void),
                            _ => None,
                        };
                        set_status.set(s);
                    }
                >
                    <option value="">"All Statuses"</option>
                    <option value="draft">"Draft"</option>
                    <option value="sent">"Sent"</option>
                    <option value="partial">"Partial"</option>
                    <option value="paid">"Paid"</option>
                    <option value="overdue">"Overdue"</option>
                    <option value="void">"Void"</option>
                </select>
            </div>

            <Transition fallback=move || view!{ <div>"Loading invoices..."</div> }>
                {move || match invoices.get() {
                    Some(Ok(list)) => view!{ <InvoicesTable items=list invoices_res=invoices.clone()/> }.into_view(),
                    Some(Err(e)) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                    None => view!{ <div/> }.into_view(),
                }}
            </Transition>
        </div>
    }
}

#[component]
fn InvoicesTable(items: Vec<Invoice>, invoices_res: Resource<(Option<InvoiceStatus>, String), Result<Vec<Invoice>, String>>) -> impl IntoView {
    let toaster = use_context::<ToastContext>();
    let mark_sent = create_action(move |id: &Uuid| {
        let id = *id;
        let toaster = toaster.clone();
        let invoices_res = invoices_res.clone();
        async move {
            match api::update_invoice_status(id, InvoiceStatus::Sent).await {
                Ok(_) => { if let Some(t) = toaster { t.push("Invoice marked sent", ToastKind::Success); } invoices_res.refetch(); }
                Err(e) => { if let Some(t) = toaster { t.push(e, ToastKind::Error); } }
            }
        }
    });
    view! {
        <Table>
            <thead>
                <tr class="text-left border-b bg-gray-50">
                    <th class="py-2 px-3 text-gray-600">"Number"</th>
                    <th class="py-2 px-3 text-gray-600">"Date"</th>
                    <th class="py-2 px-3 text-gray-600">"Due"</th>
                    <th class="py-2 px-3 text-gray-600">"Total"</th>
                    <th class="py-2 px-3 text-gray-600">"Balance"</th>
                    <th class="py-2 px-3 text-gray-600">"Status"</th>
                    <th class="py-2 px-3 text-gray-600 text-right">"Actions"</th>
                </tr>
            </thead>
            <tbody>
                {items.into_iter().map(|inv| view!{
                    <tr class="border-b hover:bg-gray-50 dark:hover:bg-gray-800">
                        <td class="py-2 px-3"><A class="text-akowe-blue-600 hover:underline" href=format!("/invoices/{}", inv.id)>{inv.invoice_number.clone()}</A></td>
                        <td class="py-2 px-3">{inv.invoice_date.to_string()}</td>
                        <td class="py-2 px-3">{inv.due_date.to_string()}</td>
                        <td class="py-2 px-3">{format_money(&inv.total_amount)}</td>
                        <td class="py-2 px-3">{format_money(&inv.balance)}</td>
                        <td class="py-2 px-3"><span class="text-xs px-2 py-1 rounded bg-gray-100 text-gray-700 border">{format!("{:?}", inv.status).to_lowercase()}</span></td>
                        <td class="py-2 px-3 text-right">
                            {if matches!(inv.status, InvoiceStatus::Overdue) {
                                let id = inv.id;
                                view!{ <button class="text-xs text-blue-700 hover:underline" on:click=move |_| mark_sent.dispatch(id)>"Mark Sent"</button> }.into_view()
                            } else { view!{ <span class="text-xs text-gray-400">"—"</span> }.into_view() }}
                        </td>
                    </tr>
                }).collect_view()}
            </tbody>
        </Table>
    }
}
