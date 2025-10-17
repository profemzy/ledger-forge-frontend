use leptos::*;
use leptos_router::{use_params_map, A};
use uuid::Uuid;

use crate::api::bills as api;
use crate::types::bills::BillWithLineItems;
use crate::utils::format::format_money;

#[component]
pub fn BillDetail() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned());
    let bill_res = create_resource(
        move || id(),
        |id_opt| async move {
            let id_str = id_opt.ok_or_else(|| "Missing id".to_string())?;
            let id = Uuid::parse_str(&id_str).map_err(|_| "Invalid id".to_string())?;
            api::get_bill(id).await
        }
    );

    view! {
        <div class="p-6 space-y-4">
            <Transition fallback=move || view!{ <div>"Loading bill..."</div> }>
                {move || match bill_res.get() {
                    Some(Ok(b)) => view!{ <BillView b=b/> }.into_view(),
                    Some(Err(e)) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                    None => view!{ <div/> }.into_view(),
                }}
            </Transition>
        </div>
    }
}

#[component]
fn BillView(b: BillWithLineItems) -> impl IntoView {
    view! {
        <div class="space-y-4">
            <div class="text-sm text-gray-600"><A class="text-akowe-blue-600 hover:underline" href="/bills">"Bills"</A> " / " {b.bill.bill_number.clone().unwrap_or_else(|| b.bill.id.to_string())}</div>
            <div class="bg-white dark:bg-gray-900 rounded shadow p-4">
                <div class="flex items-center justify-between">
                    <div>
                        <div class="text-xl font-semibold">{"Bill "}{b.bill.bill_number.clone().unwrap_or_else(|| b.bill.id.to_string())}</div>
                        <div class="text-gray-600">{"Date: "}{b.bill.bill_date.to_string()} {" • Due: "}{b.bill.due_date.to_string()}</div>
                        <div class="mt-1">
                            <span class="text-xs px-2 py-1 rounded bg-gray-100 dark:bg-gray-800 text-gray-700 border">{"Status: "}{format!("{:?}", b.bill.status).to_lowercase()}</span>
                            <span class="ml-2 text-xs px-2 py-1 rounded bg-blue-100 text-blue-700 border">{"Total: "}{format_money(&b.bill.total_amount)}</span>
                            <span class="ml-2 text-xs px-2 py-1 rounded bg-yellow-100 text-yellow-700 border">{"Balance: "}{format_money(&b.bill.balance)}</span>
                        </div>
                    </div>
                </div>
            </div>

            <div class="bg-white dark:bg-gray-900 rounded shadow">
                <table class="w-full border-collapse">
                    <thead>
                        <tr class="text-left border-b">
                            <th class="py-2 px-3 text-gray-600">"Line"</th>
                            <th class="py-2 px-3 text-gray-600">"Description"</th>
                            <th class="py-2 px-3 text-gray-600">"Amount"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {b.line_items.into_iter().map(|li| view!{
                            <tr class="border-b">
                                <td class="py-2 px-3">{li.line_number}</td>
                                <td class="py-2 px-3">{li.description.clone().unwrap_or_else(|| "—".into())}</td>
                                <td class="py-2 px-3">{format_money(&li.amount)}</td>
                            </tr>
                        }).collect_view()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

