use leptos::*;
use crate::api::reporting as api;
use crate::types::reporting::ProfitLossStatement;
use crate::utils::format::format_money;

fn default_period() -> (String, String) {
    let d = js_sys::Date::new_0();
    let year = d.get_utc_full_year();
    (format!("{:04}-01-01", year), format!("{:04}-12-31", year))
}

#[component]
pub fn ProfitLossPage() -> impl IntoView {
    let (start, end) = default_period();
    let (start_date, set_start) = create_signal(start);
    let (end_date, set_end) = create_signal(end);
    let report = create_resource(
        move || (start_date.get(), end_date.get()),
        |(s, e)| async move { api::get_profit_loss(&s, &e).await },
    );

    view! {
        <div class="p-6 space-y-4">
            <div class="flex items-center justify-between">
                <h1 class="text-2xl font-semibold">"Profit & Loss"</h1>
                <div class="space-x-2">
                    <label class="text-sm text-gray-600 mr-1">"Start"</label>
                    <input class="border rounded px-3 py-2" type="date" prop:value=move || start_date.get() on:input=move |e| set_start.set(event_target_value(&e)) />
                    <label class="text-sm text-gray-600 mx-1">"End"</label>
                    <input class="border rounded px-3 py-2" type="date" prop:value=move || end_date.get() on:input=move |e| set_end.set(event_target_value(&e)) />
                </div>
            </div>

            <Transition fallback=move || view!{ <div>"Loading..."</div> }>
                {move || match report.get() {
                    Some(Ok(pl)) => view!{ <ProfitLossTable pl=pl/> }.into_view(),
                    Some(Err(e)) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                    None => view!{ <div/> }.into_view(),
                }}
            </Transition>
        </div>
    }
}

#[component]
fn ProfitLossTable(pl: ProfitLossStatement) -> impl IntoView {
    view! {
        <div class="bg-white rounded shadow p-4 space-y-6">
            <div>
                <div class="text-lg font-semibold mb-2">"Revenue"</div>
                <table class="w-full border-collapse">
                    <tbody>
                        {pl.revenue_entries.iter().map(|e| view!{
                            <tr>
                                <td class="py-1 px-3">{format!("{} - {}", e.account_code, e.account_name)}</td>
                                <td class="py-1 px-3 text-right">{format_money(&e.amount)}</td>
                            </tr>
                        }).collect_view()}
                        <tr class="border-t">
                            <td class="py-2 px-3 font-semibold">"Total Revenue"</td>
                            <td class="py-2 px-3 text-right font-semibold">{format_money(&pl.total_revenue)}</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <div>
                <div class="text-lg font-semibold mb-2">"Expenses"</div>
                <table class="w-full border-collapse">
                    <tbody>
                        {pl.expense_entries.iter().map(|e| view!{
                            <tr>
                                <td class="py-1 px-3">{format!("{} - {}", e.account_code, e.account_name)}</td>
                                <td class="py-1 px-3 text-right">{format_money(&e.amount)}</td>
                            </tr>
                        }).collect_view()}
                        <tr class="border-t">
                            <td class="py-2 px-3 font-semibold">"Total Expenses"</td>
                            <td class="py-2 px-3 text-right font-semibold">{format_money(&pl.total_expenses)}</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <div class="text-xl font-semibold text-right">
                {"Net Income: "}{format_money(&pl.net_income)}
            </div>
        </div>
    }
}
