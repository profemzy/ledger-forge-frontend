use leptos::*;
use leptos_router::A;
use uuid::Uuid;

use crate::api::bills as api;
use crate::api::contacts as contacts_api;
use crate::types::bills::{Bill, BillStatus};
use crate::types::contacts::Contact;
use crate::components::ui::{ButtonLink, Table};
use crate::utils::format::format_money;

#[component]
pub fn BillsList() -> impl IntoView {
    let (status, set_status) = create_signal(None::<BillStatus>);
    let (vendor_id_str, set_vendor_id_str) = create_signal(String::new());

    let bills = create_resource(
        move || (status.get(), vendor_id_str.get()),
        |(s, vid)| async move {
            let vendor_id = if vid.trim().is_empty() { None } else { Uuid::parse_str(&vid).ok() };
            api::list_bills(vendor_id, s, Some(100)).await
        },
    );

    let vendors = create_resource(|| (), |_| async move { contacts_api::list_vendors().await });

    view! {
        <div class="p-6">
            <div class="flex items-center justify-between mb-4">
                <h1 class="text-2xl font-semibold">"Bills"</h1>
                <div class="flex items-center gap-2">
                    <ButtonLink href="/bill-payments/new" variant="secondary">{"Record Bill Payment"}</ButtonLink>
                    <ButtonLink href="/bills/new" variant="primary">{"New Bill"}</ButtonLink>
                </div>
            </div>

            <div class="flex gap-4 mb-4 items-center">
                <input class="border rounded px-3 py-2 w-72" type="text" placeholder="Filter by Vendor ID (UUID)"
                    prop:value=move || vendor_id_str.get()
                    on:input=move |e| set_vendor_id_str.set(event_target_value(&e))
                />
                <select class="border rounded px-2 py-1"
                    on:change=move |e| {
                        let v = event_target_value(&e);
                        let s = match v.as_str() {
                            "open" => Some(BillStatus::Open),
                            "paid" => Some(BillStatus::Paid),
                            "partial" => Some(BillStatus::Partial),
                            "void" => Some(BillStatus::Void),
                            _ => None,
                        };
                        set_status.set(s);
                    }
                >
                    <option value="">"All Statuses"</option>
                    <option value="open">"Open"</option>
                    <option value="partial">"Partial"</option>
                    <option value="paid">"Paid"</option>
                    <option value="void">"Void"</option>
                </select>
            </div>

            <Transition fallback=move || view!{ <div>"Loading bills..."</div> }>
                {move || match (bills.get(), vendors.get()) {
                    (Some(Ok(list)), vopt) => {
                        let vendor_map = vopt.and_then(|r| r.ok()).map(|v| v.into_iter().map(|c: Contact| (c.id, c.name)).collect::<std::collections::HashMap<_, _>>());
                        view!{ <BillsTable items=list vendors=vendor_map/> }.into_view()
                    }
                    (Some(Err(e)), _) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                    _ => view!{ <div/> }.into_view(),
                }}
            </Transition>
        </div>
    }
}

#[component]
fn BillsTable(items: Vec<Bill>, vendors: Option<std::collections::HashMap<Uuid, String>>) -> impl IntoView {
    view! {
        <Table>
            <thead>
                <tr class="text-left border-b bg-gray-50 dark:bg-gray-800">
                    <th class="py-2 px-3 text-gray-600">"Bill #"</th>
                    <th class="py-2 px-3 text-gray-600">"Vendor"</th>
                    <th class="py-2 px-3 text-gray-600">"Date"</th>
                    <th class="py-2 px-3 text-gray-600">"Due"</th>
                    <th class="py-2 px-3 text-gray-600">"Total"</th>
                    <th class="py-2 px-3 text-gray-600">"Balance"</th>
                    <th class="py-2 px-3 text-gray-600">"Status"</th>
                </tr>
            </thead>
            <tbody>
                {items.into_iter().map(|b| {
                    let vname = vendors.as_ref().and_then(|m| m.get(&b.vendor_id)).cloned().unwrap_or_else(|| b.vendor_id.to_string());
                    view!{
                        <tr class="border-b hover:bg-gray-50 dark:hover:bg-gray-800">
                            <td class="py-2 px-3 font-mono text-sm"><A class="text-akowe-blue-600 hover:underline" href=format!("/bills/{}", b.id)>{b.bill_number.clone().unwrap_or_else(|| "—".into())}</A></td>
                            <td class="py-2 px-3">{vname}</td>
                            <td class="py-2 px-3">{b.bill_date.to_string()}</td>
                            <td class="py-2 px-3">{b.due_date.to_string()}</td>
                            <td class="py-2 px-3">{format_money(&b.total_amount)}</td>
                            <td class="py-2 px-3">{format_money(&b.balance)}</td>
                            <td class="py-2 px-3 text-xs"><span class="px-2 py-1 rounded bg-gray-100 dark:bg-gray-800 border">{format!("{:?}", b.status).to_lowercase()}</span></td>
                        </tr>
                    }
                }).collect_view()}
            </tbody>
        </Table>
    }
}
