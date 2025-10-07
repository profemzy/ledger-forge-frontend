use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::{Dashboard, Login, NotFound};
use crate::state::AuthContext;

#[component]
pub fn App() -> impl IntoView {
    // Provide meta context for SEO
    provide_meta_context();
    
    // Global auth state
    let (user, set_user) = create_signal(None);
    provide_context(AuthContext { user, set_user });
    
    view! {
        <Stylesheet id="leptos" href="/pkg/akowe.css"/>
        <Title text="Akowe - The Documenter"/>
        <Meta name="description" content="Professional accounting system built with Rust"/>
        
        <Router>
            <Routes>
                <Route path="/" view=|| view! { <Redirect path="/dashboard"/> }/>
                <Route path="/login" view=Login/>
                <Route path="/dashboard" view=Dashboard/>
                <Route path="/*" view=NotFound/>
            </Routes>
        </Router>
    }
}