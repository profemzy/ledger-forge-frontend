use leptos::*;
use leptos_router::A;

#[component]
pub fn ReportsIndex() -> impl IntoView {
    view! {
        <div class="p-6 space-y-4">
            <h1 class="text-2xl font-semibold">"Reports"</h1>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                <ReportCard title="Trial Balance" desc="Debits vs Credits as of date" href="/reports/trial-balance"/>
                <ReportCard title="Profit & Loss" desc="Revenue, Expenses, Net Income" href="/reports/profit-loss"/>
                <ReportCard title="Balance Sheet" desc="Assets, Liabilities, Equity" href="/reports/balance-sheet"/>
                <ReportCard title="A/R Aging" desc="Receivables by aging buckets" href="/reports/ar-aging"/>
            </div>
        </div>
    }
}

#[component]
fn ReportCard(title: &'static str, desc: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <A href=href class="block bg-white rounded shadow p-4 hover:shadow-md transition">
            <div class="text-lg font-semibold">{title}</div>
            <div class="text-sm text-gray-600">{desc}</div>
        </A>
    }
}
