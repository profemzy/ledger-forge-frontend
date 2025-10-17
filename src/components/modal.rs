use leptos::*;
use leptos::ChildrenFn;

#[component]
pub fn Modal(
    show: ReadSignal<bool>,
    on_close: Callback<()>,
    title: String,
    children: Children,
    actions: ChildrenFn,
) -> impl IntoView {
    let close = move |_| on_close.call(());
    view! {
        <div class=move || if show.get() { "fixed inset-0 z-50 flex items-center justify-center" } else { "hidden" }>
            <div class="absolute inset-0 bg-black/40" on:click=close></div>
            <div class="relative bg-white dark:bg-gray-900 rounded shadow-lg w-full max-w-md p-4">
                <div class="text-lg font-semibold mb-2">{title.clone()}</div>
                <div class="mb-4">{children()}</div>
                <div class="flex items-center justify-end gap-2">{actions()}</div>
            </div>
        </div>
    }
}
