use leptos::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="p-6">
            <h1 class="text-xl font-semibold">"404 - Page Not Found"</h1>
            <p class="mt-2">"The page you are looking for does not exist."</p>
        </div>
    }
}

