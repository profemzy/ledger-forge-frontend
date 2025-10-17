use leptos::*;
use crate::api::reporting as api;
use crate::types::reporting::TrialBalance;
use crate::utils::format::format_money;

fn today_ymd() -> String {
    let d = js_sys::Date::new_0();
    let year = d.get_utc_full_year();
    let month = (d.get_utc_month() + 1) as u32;
    let day = d.get_utc_date() as u32;
    format!("{:04}-{:02}-{:02}", year, month, day)
}

#[component]
pub fn TrialBalancePage() -> impl IntoView {
    let (as_of, set_as_of) = create_signal(today_ymd());
    let report = create_resource(move || as_of.get(), |date| async move { api::get_trial_balance(&date).await });

    view! {
        <div class="p-6 space-y-4">
            <div class="flex items-center justify-between">
                <h1 class="text-2xl font-semibold">"Trial Balance"</h1>
                <div>
                    <label class="text-sm text-gray-600 mr-2">"As of"</label>
                    <input class="border rounded px-3 py-2" type="date" prop:value=move || as_of.get() on:input=move |e| set_as_of.set(event_target_value(&e)) />
                </div>
            </div>

            <Transition fallback=move || view!{ <div>"Loading..."</div> }>
                {move || match report.get() {
                    Some(Ok(tb)) => view!{ <TrialBalanceTable tb=tb/> }.into_view(),
                    Some(Err(e)) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                    None => view!{ <div/> }.into_view(),
                }}
            </Transition>
        </div>
    }
}

#[component]
fn TrialBalanceTable(tb: TrialBalance) -> impl IntoView {
    view! {
        <div class="bg-white rounded shadow">
            <table class="w-full border-collapse">
                <thead>
                    <tr class="text-left border-b">
                        <th class="py-2 px-3 text-gray-600">"Code"</th>
                        <th class="py-2 px-3 text-gray-600">"Name"</th>
                        <th class="py-2 px-3 text-gray-600">"Type"</th>
                        <th class="py-2 px-3 text-gray-600">"Debit"</th>
                        <th class="py-2 px-3 text-gray-600">"Credit"</th>
                    </tr>
                </thead>
                <tbody>
                    {tb.entries.iter().map(|e| view!{
                        <tr class="border-b">
                            <td class="py-2 px-3 font-mono text-sm">{e.account_code.clone()}</td>
                            <td class="py-2 px-3">{e.account_name.clone()}</td>
                            <td class="py-2 px-3">{e.account_type.clone()}</td>
                            <td class="py-2 px-3">{format_money(&e.debit)}</td>
                            <td class="py-2 px-3">{format_money(&e.credit)}</td>
                        </tr>
                    }).collect_view()}
                </tbody>
                <tfoot>
                    <tr>
                        <td class="py-2 px-3" colspan="3"><span class="text-sm text-gray-600">"Totals"</span></td>
                        <td class="py-2 px-3 font-semibold">{format_money(&tb.total_debits)}</td>
                        <td class="py-2 px-3 font-semibold">{format_money(&tb.total_credits)}</td>
                    </tr>
                </tfoot>
            </table>
            <div class={if tb.is_balanced {"p-3 text-green-700"} else {"p-3 text-red-700"}}>{if tb.is_balanced {"Balanced"} else {"Unbalanced"}}</div>
        </div>
    }
}
