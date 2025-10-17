use leptos::*;
use crate::api::reporting as api;
use crate::types::reporting::BalanceSheet;
use crate::utils::format::format_money;

fn today_ymd() -> String {
    let d = js_sys::Date::new_0();
    let year = d.get_utc_full_year();
    let month = (d.get_utc_month() + 1) as u32;
    let day = d.get_utc_date() as u32;
    format!("{:04}-{:02}-{:02}", year, month, day)
}

#[component]
pub fn BalanceSheetPage() -> impl IntoView {
    let (as_of, set_as_of) = create_signal(today_ymd());
    let report = create_resource(move || as_of.get(), |date| async move { api::get_balance_sheet(&date).await });

    view! {
        <div class="p-6 space-y-4">
            <div class="flex items-center justify-between">
                <h1 class="text-2xl font-semibold">"Balance Sheet"</h1>
                <div>
                    <label class="text-sm text-gray-600 mr-2">"As of"</label>
                    <input class="border rounded px-3 py-2" type="date" prop:value=move || as_of.get() on:input=move |e| set_as_of.set(event_target_value(&e)) />
                </div>
            </div>

            <Transition fallback=move || view!{ <div>"Loading..."</div> }>
                {move || match report.get() {
                    Some(Ok(bs)) => view!{ <BalanceSheetTable bs=bs/> }.into_view(),
                    Some(Err(e)) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                    None => view!{ <div/> }.into_view(),
                }}
            </Transition>
        </div>
    }
}

#[component]
fn BalanceSheetTable(bs: BalanceSheet) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div class="bg-white rounded shadow p-4">
                <div class="text-lg font-semibold mb-2">"Assets"</div>
                <table class="w-full border-collapse">
                    <tbody>
                        {bs.asset_entries.iter().map(|e| view!{
                            <tr>
                                <td class="py-1 px-3">{format!("{} - {}", e.account_code, e.account_name)}</td>
                                <td class="py-1 px-3 text-right">{format_money(&e.amount)}</td>
                            </tr>
                        }).collect_view()}
                        <tr class="border-t">
                            <td class="py-2 px-3 font-semibold">"Total Assets"</td>
                            <td class="py-2 px-3 text-right font-semibold">{format_money(&bs.total_assets)}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="bg-white rounded shadow p-4">
                <div class="text-lg font-semibold mb-2">"Liabilities"</div>
                <table class="w-full border-collapse">
                    <tbody>
                        {bs.liability_entries.iter().map(|e| view!{
                            <tr>
                                <td class="py-1 px-3">{format!("{} - {}", e.account_code, e.account_name)}</td>
                                <td class="py-1 px-3 text-right">{format_money(&e.amount)}</td>
                            </tr>
                        }).collect_view()}
                        <tr class="border-t">
                            <td class="py-2 px-3 font-semibold">"Total Liabilities"</td>
                            <td class="py-2 px-3 text-right font-semibold">{format_money(&bs.total_liabilities)}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="bg-white rounded shadow p-4">
                <div class="text-lg font-semibold mb-2">"Equity"</div>
                <table class="w-full border-collapse">
                    <tbody>
                        {bs.equity_entries.iter().map(|e| view!{
                            <tr>
                                <td class="py-1 px-3">{format!("{} - {}", e.account_code, e.account_name)}</td>
                                <td class="py-1 px-3 text-right">{format_money(&e.amount)}</td>
                            </tr>
                        }).collect_view()}
                        <tr class="border-t">
                            <td class="py-2 px-3 font-semibold">"Total Equity"</td>
                            <td class="py-2 px-3 text-right font-semibold">{format_money(&bs.total_equity)}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    }
}
