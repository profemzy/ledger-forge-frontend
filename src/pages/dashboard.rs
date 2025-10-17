use leptos::*;
use leptos_router::*;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use chrono::{Datelike, NaiveDate, Utc, Duration};
use chrono::Months;

use crate::state::AuthContext;
use crate::api::{invoices as inv_api, payments as pay_api, transactions as tx_api, reporting};
use crate::utils::format::format_money;
use crate::types::transactions::Transaction;
use crate::types::payments::Payment;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct NetIncomePoint {
    label: String,
    amount: Decimal,
}

fn today_ymd() -> String {
    // Use JS Date to get today's date in YYYY-MM-DD
    let d = js_sys::Date::new_0();
    let year = d.get_utc_full_year();
    let month = (d.get_utc_month() + 1) as u32;
    let day = d.get_utc_date() as u32;
    format!("{:04}-{:02}-{:02}", year, month, day)
}

#[component]
pub fn Dashboard() -> impl IntoView {
    let navigate = use_navigate();
    let auth = expect_context::<AuthContext>();

    create_effect(move |_| {
        if !auth.is_authenticated() {
            navigate("/login", Default::default());
        }
    });

    let (as_of, set_as_of) = create_signal(today_ymd());

    // Balance sheet
    let bs_res = create_resource(move || as_of.get(), |date| async move { reporting::get_balance_sheet(&date).await });

    // AR aging
    let ar_res = create_resource(move || as_of.get(), |date| async move { reporting::get_ar_aging(&date).await });

    // Overdue invoices
    let overdue_res = create_resource(|| (), |_| async move { inv_api::get_overdue_invoices().await });

    // Unapplied payments
    let unapplied_res = create_resource(|| (), |_| async move { pay_api::get_unapplied_payments().await });

    // Recent transactions & payments
    let recent_transactions = create_resource(|| (), |_| async move { tx_api::list_transactions(None, None, Some(5)).await });
    let recent_payments = create_resource(|| (), |_| async move { pay_api::list_payments().await.map(|mut list| { list.sort_by(|a, b| b.payment_date.cmp(&a.payment_date)); list.into_iter().take(5).collect::<Vec<_>>() }) });
    let income_history = create_resource(|| (), |_| async move { fetch_net_income_history(6).await });

    view! {
        <div class="p-6 space-y-6">
            <div class="flex items-center justify-between">
                <h1 class="text-2xl font-semibold">"Dashboard"</h1>
                {move || auth.get_user().map(|u| view!{ <p class="text-gray-600">{"Welcome, "}{u.username}</p> })}
            </div>

            <div class="flex flex-wrap gap-3 items-end">
                <div>
                    <label class="block text-sm text-gray-600">"As of date"</label>
                    <input class="border rounded px-3 py-2" type="date" prop:value=move || as_of.get() on:input=move |e| set_as_of.set(event_target_value(&e)) />
                </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4">
                // Total Assets
                <div class="bg-white rounded shadow p-4">
                    <div class="text-sm text-gray-600">"Total Assets"</div>
                    <div class="text-2xl font-semibold mt-1">
                        <Transition fallback=|| view!{ <span>"—"</span> }>
                            {move || bs_res.get().map(|r| match r { Ok(bs) => format_money(&bs.total_assets), Err(_) => "—".into() })}
                        </Transition>
                    </div>
                </div>
                // Total Liabilities
                <div class="bg-white rounded shadow p-4">
                    <div class="text-sm text-gray-600">"Total Liabilities"</div>
                    <div class="text-2xl font-semibold mt-1">
                        <Transition fallback=|| view!{ <span>"—"</span> }>
                            {move || bs_res.get().map(|r| match r { Ok(bs) => format_money(&bs.total_liabilities), Err(_) => "—".into() })}
                        </Transition>
                    </div>
                </div>
                // Total Equity
                <div class="bg-white rounded shadow p-4">
                    <div class="text-sm text-gray-600">"Total Equity"</div>
                    <div class="text-2xl font-semibold mt-1">
                        <Transition fallback=|| view!{ <span>"—"</span> }>
                            {move || bs_res.get().map(|r| match r { Ok(bs) => format_money(&bs.total_equity), Err(_) => "—".into() })}
                        </Transition>
                    </div>
                </div>
                // Cash on Hand
                <div class="bg-white rounded shadow p-4">
                    <div class="text-sm text-gray-600">"Cash on Hand"</div>
                    <div class="text-2xl font-semibold mt-1">
                        <Transition fallback=|| view!{ <span>"—"</span> }>
                            {move || bs_res.get().map(|r| match r {
                                Ok(bs) => {
                                    let total_cash = bs.asset_entries.iter()
                                        .filter(|entry| {
                                            let name = entry.account_name.to_lowercase();
                                            name.contains("cash") || entry.account_code.starts_with("10")
                                        })
                                        .fold(Decimal::ZERO, |acc, entry| acc + entry.amount);
                                    format_money(&total_cash)
                                }
                                Err(_) => "—".into()
                            })}
                        </Transition>
                    </div>
                </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                // AR Outstanding
                <div class="bg-white rounded shadow p-4">
                    <div class="text-sm text-gray-600">"AR Outstanding"</div>
                    <div class="text-2xl font-semibold mt-1">
                        <Transition fallback=|| view!{ <span>"—"</span> }>
                            {move || ar_res.get().map(|r| match r { Ok(ar) => format_money(&ar.total_outstanding), Err(_) => "—".into() })}
                        </Transition>
                    </div>
                </div>
                // Overdue Invoices
                <div class="bg-white rounded shadow p-4">
                    <div class="text-sm text-gray-600">"Overdue Invoices"</div>
                    <div class="text-2xl font-semibold mt-1">
                        <Transition fallback=|| view!{ <span>"—"</span> }>
                            {move || overdue_res.get().map(|r| match r {
                                Ok(list) => {
                                    let total: Decimal = list.iter().map(|inv| inv.balance).sum();
                                    format!("{} ({})", format_money(&total), list.len())
                                }
                                Err(_) => "—".into()
                            })}
                        </Transition>
                    </div>
                    <div class="mt-2"><A class="text-akowe-blue-600 hover:underline" href="/invoices">"View invoices"</A></div>
                </div>
                // Unapplied Payments
                <div class="bg-white rounded shadow p-4">
                    <div class="text-sm text-gray-600">"Unapplied Payments"</div>
                    <div class="text-2xl font-semibold mt-1">
                        <Transition fallback=|| view!{ <span>"—"</span> }>
                            {move || unapplied_res.get().map(|r| match r {
                                Ok(list) => {
                                    let total: Decimal = list.iter().map(|p| p.unapplied_amount.unwrap_or_default()).sum();
                                    format!("{} ({})", format_money(&total), list.len())
                                }
                                Err(_) => "—".into()
                            })}
                        </Transition>
                    </div>
                    <div class="mt-2"><A class="text-akowe-blue-600 hover:underline" href="/payments">"View payments"</A></div>
                </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
                <QuickLink title="New Transaction" href="/transactions/new" desc="Record a journal entry"/>
                <QuickLink title="New Invoice" href="/invoices/new" desc="Bill a customer"/>
                <QuickLink title="New Payment" href="/payments/new" desc="Record customer payment"/>
                <QuickLink title="Accounts" href="/accounts" desc="Manage chart of accounts"/>
                <QuickLink title="Reports" href="/reports" desc="Trial Balance, P&L, Balance Sheet"/>
            </div>

            <NetIncomeChart res=income_history.clone()/ >

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                <RecentTransactionsView res=recent_transactions.clone()/ >
                <RecentPaymentsView res=recent_payments.clone()/ >
            </div>
        </div>
    }
}

#[component]
fn QuickLink(title: &'static str, href: &'static str, desc: &'static str) -> impl IntoView {
    view! {
        <A href=href class="block bg-white rounded shadow p-4 hover:shadow-md transition">
            <div class="text-lg font-semibold">{title}</div>
            <div class="text-sm text-gray-600">{desc}</div>
        </A>
    }
}

#[component]
fn RecentTransactionsView(res: Resource<(), Result<Vec<Transaction>, String>>) -> impl IntoView {
    view! {
        <div class="bg-white rounded shadow p-4">
            <div class="text-lg font-semibold mb-2">"Recent Transactions"</div>
            <Transition fallback=move || view!{ <div>"Loading..."</div> }>
                {move || match res.get() {
                    Some(Ok(list)) => {
                        if list.is_empty() {
                            view!{ <div class="text-gray-600">"No recent transactions."</div> }.into_view()
                        } else {
                            let items = list.clone();
                            view!{
                                <table class="w-full border-collapse">
                                    <thead>
                                        <tr class="text-left border-b">
                                            <th class="py-2 px-3 text-gray-600">"Date"</th>
                                            <th class="py-2 px-3 text-gray-600">"Description"</th>
                                            <th class="py-2 px-3 text-gray-600">"Status"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {items.into_iter().map(|tx| view!{
                                            <tr class="border-b">
                                                <td class="py-2 px-3">{tx.transaction_date.to_string()}</td>
                                                <td class="py-2 px-3"><A class="text-akowe-blue-600 hover:underline" href=format!("/transactions/{}", tx.id)>{tx.description.clone().unwrap_or_else(|| "—".into())}</A></td>
                                                <td class="py-2 px-3 text-sm">{format!("{:?}", tx.status).to_lowercase()}</td>
                                            </tr>
                                        }).collect_view()}
                                    </tbody>
                                </table>
                            }.into_view()
                        }
                    }
                    Some(Err(e)) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                    None => view!{ <div/> }.into_view(),
                }}
            </Transition>
        </div>
    }
}

#[component]
fn RecentPaymentsView(res: Resource<(), Result<Vec<Payment>, String>>) -> impl IntoView {
    view! {
        <div class="bg-white rounded shadow p-4">
            <div class="text-lg font-semibold mb-2">"Recent Payments"</div>
            <Transition fallback=move || view!{ <div>"Loading..."</div> }>
                {move || match res.get() {
                    Some(Ok(items)) => {
                        if items.is_empty() {
                            view!{ <div class="text-gray-600">"No recent payments."</div> }.into_view()
                        } else {
                            let entries = items;
                            view!{
                                <table class="w-full border-collapse">
                                    <thead>
                                        <tr class="text-left border-b">
                                            <th class="py-2 px-3 text-gray-600">"Date"</th>
                                            <th class="py-2 px-3 text-gray-600">"Number"</th>
                                            <th class="py-2 px-3 text-gray-600">"Amount"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {entries.into_iter().map(|p| {
                                            let number = p.payment_number.unwrap_or_else(|| "—".into());
                                            view!{
                                                <tr class="border-b">
                                                    <td class="py-2 px-3">{p.payment_date.to_string()}</td>
                                                    <td class="py-2 px-3 font-mono text-sm"><A class="text-akowe-blue-600 hover:underline" href=format!("/payments/{}", p.id)>{number}</A></td>
                                                    <td class="py-2 px-3">{format_money(&p.amount)}</td>
                                                </tr>
                                            }
                                        }).collect_view()}
                                    </tbody>
                                </table>
                            }.into_view()
                        }
                    }
                    Some(Err(e)) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                    None => view!{ <div/> }.into_view(),
                }}
            </Transition>
        </div>
    }
}

#[component]
fn NetIncomeChart(res: Resource<(), Result<Vec<NetIncomePoint>, String>>) -> impl IntoView {
    view! {
        <div class="bg-white rounded shadow p-4">
            <div class="text-lg font-semibold mb-2">"Net Income (last 6 months)"</div>
            <Transition fallback=move || view!{ <div>"Loading..."</div> }>
                {move || match res.get() {
                    Some(Ok(points)) => {
                        if points.is_empty() {
                            view!{ <div class="text-gray-600">"No data."</div> }.into_view()
                        } else {
                            let entries = points.clone();
                            let max_abs = entries.iter().map(|p| p.amount.abs()).fold(Decimal::ZERO, |acc, x| if x > acc { x } else { acc });
                            let max_f = max_abs.to_f64().unwrap_or(1.0).max(0.01);
                            view!{
                                <div class="flex items-end gap-3 h-40">
                                    {entries.into_iter().map(|pt| {
                                        let height_px = (pt.amount.abs().to_f64().unwrap_or(0.0) / max_f * 120.0).max(4.0);
                                        let bar_class = if pt.amount >= Decimal::ZERO { "bg-green-500" } else { "bg-red-500" };
                                        view!{
                                            <div class="flex flex-col items-center gap-1 w-12">
                                                <div class="w-full bg-gray-200 rounded-sm h-32 flex items-end">
                                                    <div class=bar_class style=format!("height: {:.1}px; width: 100%;", height_px)></div>
                                                </div>
                                                <div class="text-xs text-gray-600">{pt.label.clone()}</div>
                                                <div class="text-xs font-mono">{format_money(&pt.amount)}</div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_view()
                        }
                    }
                    Some(Err(e)) => view!{ <div class="text-red-600">{e}</div> }.into_view(),
                    None => view!{ <div/> }.into_view(),
                }}
            </Transition>
        </div>
    }
}

fn last_n_months(n: usize) -> Vec<(String, NaiveDate, NaiveDate)> {
    let today = Utc::now().date_naive();
    let base = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
    (0..n).rev().filter_map(|i| {
        base.checked_sub_months(Months::new(i as u32)).map(|start| {
            let next = start.checked_add_months(Months::new(1)).unwrap();
            let end = next - Duration::days(1);
            (start.format("%b").to_string(), start, end)
        })
    }).collect()
}

async fn fetch_net_income_history(months: usize) -> Result<Vec<NetIncomePoint>, String> {
    let mut points = Vec::new();
    for (label, start, end) in last_n_months(months) {
        let start_str = start.to_string();
        let end_str = end.to_string();
        let pl = reporting::get_profit_loss(&start_str, &end_str).await?;
        points.push(NetIncomePoint { label, amount: pl.net_income });
    }
    Ok(points)
}
