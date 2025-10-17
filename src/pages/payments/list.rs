use leptos::*;
use uuid::Uuid;
use leptos_router::{A, use_query_map};

use crate::api::payments as api;
use crate::api::contacts as contacts_api;
use crate::types::payments::Payment;
use crate::utils::format::format_money;
use crate::types::contacts::Contact;

#[component]
pub fn PaymentsList() -> impl IntoView {
    let (show_unapplied, set_show_unapplied) = create_signal(false);
    let (customer_id_str, set_customer_id_str) = create_signal(String::new());
    let query_map = use_query_map();

    let payments = create_resource(
        move || (show_unapplied.get(), customer_id_str.get()),
        |(unapplied, cid_str)| async move {
            let mut items = if unapplied { api::get_unapplied_payments().await? } else { api::list_payments().await? };
            if let Ok(cid) = Uuid::parse_str(&cid_str) {
                items = items.into_iter().filter(|p| p.customer_id == cid).collect();
            }
            Ok::<_, String>(items)
        },
    );

    // Initialize filters from query params (?unapplied=true&customer=<uuid>)
    create_effect(move |_| {
        let qs_unapplied = query_map.with(|q| q.get("unapplied").cloned());
        if let Some(u) = qs_unapplied { if u.to_lowercase() == "true" { set_show_unapplied.set(true); } }
        let qs_customer = query_map.with(|q| q.get("customer").cloned());
        if let Some(cid) = qs_customer { set_customer_id_str.set(cid); }
    });

    // Contacts for mapping customer names
    let contacts_res = create_resource(|| (), |_| async move { contacts_api::list_contacts().await });

    view! {
        <div class="p-6">
            <div class="flex items-center justify-between mb-4">
                <h1 class="text-2xl font-semibold">"Payments"</h1>
                <a href="/payments/new" class="bg-blue-600 text-white px-4 py-2 rounded">"New Payment"</a>
            </div>

            <div class="flex gap-4 mb-4 items-center">
                <label class="inline-flex items-center gap-2">
                    <input type="checkbox" prop:checked=move || show_unapplied.get() on:change=move |_| set_show_unapplied.set(!show_unapplied.get()) />
                    <span>"Unapplied only"</span>
                </label>
                <input class="border rounded px-3 py-2 w-72" type="text" placeholder="Filter by Customer ID (UUID)"
                    prop:value=move || customer_id_str.get()
                    on:input=move |e| set_customer_id_str.set(event_target_value(&e))
                />
            </div>

            <Transition fallback=move || view!{ <div>"Loading payments..."</div> }>
                {move || match (payments.get(), contacts_res.get()) {
                    (Some(Ok(list)), contacts) => {
                        let contact_map = contacts.and_then(|r| r.ok()).map(|v| v.into_iter().map(|c: Contact| (c.id, c.name)).collect::<std::collections::HashMap<_, _>>());
                        view!{ <PaymentsTable items=list contacts=contact_map/> }.into_view()
                    }
                    (Some(Err(e)), _) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                    _ => view!{ <div/> }.into_view(),
                }}
            </Transition>
        </div>
    }
}

#[component]
fn PaymentsTable(items: Vec<Payment>, contacts: Option<std::collections::HashMap<Uuid, String>>) -> impl IntoView {
    view! {
        <table class="w-full border-collapse bg-white rounded shadow">
            <thead>
                <tr class="text-left border-b">
                    <th class="py-2 px-3 text-gray-600">"Date"</th>
                    <th class="py-2 px-3 text-gray-600">"Number"</th>
                    <th class="py-2 px-3 text-gray-600">"Customer"</th>
                    <th class="py-2 px-3 text-gray-600">"Amount"</th>
                    <th class="py-2 px-3 text-gray-600">"Unapplied"</th>
                    <th class="py-2 px-3 text-gray-600">"Method"</th>
                </tr>
            </thead>
            <tbody>
                {items.into_iter().map(|p| {
                    let cname = contacts.as_ref().and_then(|m| m.get(&p.customer_id)).cloned().unwrap_or_else(|| p.customer_id.to_string());
                    view!{
                        <tr class="border-b hover:bg-gray-50">
                            <td class="py-2 px-3">{p.payment_date.to_string()}</td>
                            <td class="py-2 px-3 font-mono text-sm"><A class="text-akowe-blue-600 hover:underline" href=format!("/payments/{}", p.id)>{p.payment_number.unwrap_or_else(|| "—".into())}</A></td>
                            <td class="py-2 px-3">{cname}</td>
                            <td class="py-2 px-3">{format_money(&p.amount)}</td>
                            <td class="py-2 px-3">{format_money(&p.unapplied_amount.unwrap_or_default())}</td>
                            <td class="py-2 px-3">{p.payment_method.clone()}</td>
                        </tr>
                    }
                }).collect_view()}
            </tbody>
        </table>
    }
}
