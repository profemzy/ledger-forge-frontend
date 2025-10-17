use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::auth::ProtectedRoute;
use crate::pages::{Dashboard, Login, NotFound};
use crate::components::layout::Layout;
use crate::pages::transactions::{TransactionsList, TransactionCreate, TransactionDetail};
use crate::pages::invoices::{InvoicesList, InvoiceCreate, InvoiceDetail};
use crate::pages::payments::{PaymentsList, PaymentCreate};
use crate::pages::reporting::{ReportsIndex, TrialBalancePage, ProfitLossPage, BalanceSheetPage, ARAgingPage};
use crate::pages::accounts::{AccountsList, AccountCreate, AccountDetail};
use crate::pages::bills::{BillsList, BillCreate, BillDetail};
use crate::pages::bill_payments::BillPaymentCreate;
use crate::state::AuthContext;

#[component]
pub fn App() -> impl IntoView {
    // Provide meta context for SEO
    provide_meta_context();
    
    // Global auth state
    let (user, set_user) = create_signal(None);
    provide_context(AuthContext { user, set_user });
    
    view! {
        <Title text="Akowe - The Documenter"/>
        <Meta name="description" content="Professional accounting system built with Rust"/>
        
        <Router>
            <Routes>
                <Route path="/" view=|| view! { <Redirect path="/dashboard"/> }/>
                <Route path="/login" view=Login/>
                <Route path="" view=ProtectedRoute>
                    <Route path="" view=Layout>
                        <Route path="/dashboard" view=Dashboard/>
                        <Route path="/accounts" view=AccountsList/>
                        <Route path="/accounts/new" view=AccountCreate/>
                        <Route path="/accounts/:id" view=AccountDetail/>
                        <Route path="/transactions" view=TransactionsList/>
                        <Route path="/transactions/new" view=TransactionCreate/>
                        <Route path="/transactions/:id" view=TransactionDetail/>
                        <Route path="/invoices" view=InvoicesList/>
                        <Route path="/invoices/new" view=InvoiceCreate/>
                        <Route path="/invoices/:id" view=InvoiceDetail/>
                        <Route path="/payments" view=PaymentsList/>
                        <Route path="/payments/new" view=PaymentCreate/>
                        <Route path="/payments/:id" view=crate::pages::payments::detail::PaymentDetail/>
                        <Route path="/bills" view=BillsList/>
                        <Route path="/bills/new" view=BillCreate/>
                        <Route path="/bills/:id" view=BillDetail/>
                        <Route path="/bill-payments/new" view=BillPaymentCreate/>
                        <Route path="/reports" view=ReportsIndex/>
                        <Route path="/reports/trial-balance" view=TrialBalancePage/>
                        <Route path="/reports/profit-loss" view=ProfitLossPage/>
                        <Route path="/reports/balance-sheet" view=BalanceSheetPage/>
                        <Route path="/reports/ar-aging" view=ARAgingPage/>
                    </Route>
                </Route>
                <Route path="/*" view=NotFound/>
            </Routes>
        </Router>
    }
}
