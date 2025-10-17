use leptos::*;
use crate::api::reporting as api;
use crate::types::reporting::AccountsReceivableAging;
use crate::utils::format::format_money;

fn today_ymd() -> String {
    let d = js_sys::Date::new_0();
    let year = d.get_utc_full_year();
    let month = (d.get_utc_month() + 1) as u32;
    let day = d.get_utc_date() as u32;
    format!("{:04}-{:02}-{:02}", year, month, day)
}

#[component]
pub fn ARAgingPage() -> impl IntoView {
    let (as_of, set_as_of) = create_signal(today_ymd());
    let report = create_resource(move || as_of.get(), |date| async move { api::get_ar_aging(&date).await });

    view! {
        <div class="p-6 space-y-4">
            <div class="flex items-center justify-between">
                <h1 class="text-2xl font-semibold">"A/R Aging"</h1>
                <div>
                    <label class="text-sm text-gray-600 mr-2">"As of"</label>
                    <input class="border rounded px-3 py-2" type="date" prop:value=move || as_of.get() on:input=move |e| set_as_of.set(event_target_value(&e)) />
                </div>
            </div>

            <Transition fallback=move || view!{ <div>"Loading..."</div> }>
                {move || match report.get() {
                    Some(Ok(ar)) => view!{ <AgingTable ar=ar/> }.into_view(),
                    Some(Err(e)) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                    None => view!{ <div/> }.into_view(),
                }}
            </Transition>
        </div>
    }
}

#[component]
fn AgingTable(ar: AccountsReceivableAging) -> impl IntoView {
    view! {
        <div class="bg-white rounded shadow">
            <table class="w-full border-collapse">
                <thead>
                    <tr class="text-left border-b">
                        <th class="py-2 px-3 text-gray-600">"Customer"</th>
                        <th class="py-2 px-3 text-gray-600">"Current"</th>
                        <th class="py-2 px-3 text-gray-600">"1‑30"</th>
                        <th class="py-2 px-3 text-gray-600">"31‑60"</th>
                        <th class="py-2 px-3 text-gray-600">"61‑90"</th>
                        <th class="py-2 px-3 text-gray-600">"91+"</th>
                        <th class="py-2 px-3 text-gray-600">"Total"</th>
                    </tr>
                </thead>
                <tbody>
                    {ar.buckets.iter().map(|b| view!{
                        <tr class="border-b">
                            <td class="py-2 px-3">{b.customer_name.clone()}</td>
                            <td class="py-2 px-3">{format_money(&b.current)}</td>
                            <td class="py-2 px-3">{format_money(&b.days_1_30)}</td>
                            <td class="py-2 px-3">{format_money(&b.days_31_60)}</td>
                            <td class="py-2 px-3">{format_money(&b.days_61_90)}</td>
                            <td class="py-2 px-3">{format_money(&b.days_91_plus)}</td>
                            <td class="py-2 px-3 font-semibold">{format_money(&b.total)}</td>
                        </tr>
                    }).collect_view()}
                </tbody>
                <tfoot>
                    <tr>
                        <td class="py-2 px-3 font-semibold" colspan="6">"Total Outstanding"</td>
                        <td class="py-2 px-3 font-semibold">{format_money(&ar.total_outstanding)}</td>
                    </tr>
                </tfoot>
            </table>
        </div>
    }
}
