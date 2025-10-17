use leptos::*;
use leptos_router::A;

use crate::api::accounts as api;
use crate::components::ui::{ButtonLink, Table};
use crate::types::accounts::{Account, AccountType};

#[component]
pub fn AccountsList() -> impl IntoView {
    let (account_type, set_account_type) = create_signal(None::<AccountType>);
    let (include_inactive, set_include_inactive) = create_signal(false);
    let (query, set_query) = create_signal(String::new());

    let accounts = create_resource(
        move || (account_type.get(), include_inactive.get()),
        |(t, inc)| async move { api::list_accounts(t, None, Some(inc)).await },
    );

    view! {
        <div class="p-6">
            <div class="flex items-center justify-between mb-4">
                <h1 class="text-2xl font-semibold">"Accounts"</h1>
                <ButtonLink href="/accounts/new" variant="primary">{"New Account"}</ButtonLink>
            </div>

            <div class="flex flex-wrap gap-4 mb-4 items-center">
                <input
                    class="border rounded px-3 py-2 w-72"
                    type="text"
                    placeholder="Search code or name..."
                    prop:value=move || query.get()
                    on:input=move |e| set_query.set(event_target_value(&e))
                />
                <select class="border rounded px-2 py-1"
                    on:change=move |e| {
                        let val = event_target_value(&e);
                        let t = match val.as_str() {
                            "Asset" => Some(AccountType::Asset),
                            "Liability" => Some(AccountType::Liability),
                            "Equity" => Some(AccountType::Equity),
                            "Revenue" => Some(AccountType::Revenue),
                            "Expense" => Some(AccountType::Expense),
                            _ => None,
                        };
                        set_account_type.set(t);
                    }
                >
                    <option value="">"All Types"</option>
                    <option>"Asset"</option>
                    <option>"Liability"</option>
                    <option>"Equity"</option>
                    <option>"Revenue"</option>
                    <option>"Expense"</option>
                </select>

                <label class="inline-flex items-center gap-2">
                    <input type="checkbox"
                        prop:checked=move || include_inactive.get()
                        on:change=move |_| set_include_inactive.set(!include_inactive.get())
                    />
                    <span>"Include inactive"</span>
                </label>

                <button class="ml-auto text-sm text-gray-600 hover:text-gray-800 underline"
                    on:click=move |_| {
                        set_query.set(String::new());
                        set_account_type.set(None);
                        set_include_inactive.set(false);
                    }
                >"Reset filters"</button>
            </div>

            <Transition fallback=move || view! { <div>"Loading accounts..."</div> }>
                {move || match accounts.get() {
                    Some(Ok(list)) => {
                        // client-side search filter
                        let q = query.get().to_lowercase();
                        let filtered = if q.is_empty() { list } else {
                            list.into_iter().filter(|a| a.code.to_lowercase().contains(&q) || a.name.to_lowercase().contains(&q)).collect()
                        };
                        if filtered.is_empty() {
                            view! { <div class="text-gray-600">"No accounts found."</div> }.into_view()
                        } else {
                            view! { <AccountsTable accounts=filtered/> }.into_view()
                        }
                    }
                    Some(Err(e)) => view! { <div class="text-red-600">{e}</div> }.into_view(),
                    None => view! { <div/> }.into_view(),
                }}
            </Transition>
        </div>
    }
}

#[component]
fn AccountsTable(accounts: Vec<Account>) -> impl IntoView {
    view! {
        <Table>
            <thead>
                <tr class="text-left border-b bg-gray-50">
                    <th class="py-2 px-3 text-gray-600">"Code"</th>
                    <th class="py-2 px-3 text-gray-600">"Name"</th>
                    <th class="py-2 px-3 text-gray-600">"Type"</th>
                    <th class="py-2 px-3 text-gray-600">"Status"</th>
                </tr>
            </thead>
            <tbody>
                {accounts.into_iter().map(|a| view! {
                    <tr class="border-b hover:bg-gray-50 dark:hover:bg-gray-800">
                        <td class="py-2 px-3 font-mono">{a.code.clone()}</td>
                        <td class="py-2 px-3"><A class="text-akowe-blue-600 hover:underline" href=format!("/accounts/{}", a.id)>{a.name.clone()}</A></td>
                        <td class="py-2 px-3">
                            <span class="inline-block text-xs px-2 py-1 rounded bg-gray-100 text-gray-700 border">{format!("{:?}", a.account_type)}</span>
                        </td>
                        <td class="py-2 px-3">
                            {if a.is_active { view!{ <span class="text-xs px-2 py-1 rounded bg-green-100 text-green-700 border border-green-200">"Active"</span> }.into_view() } else { view!{ <span class="text-xs px-2 py-1 rounded bg-red-100 text-red-700 border border-red-200">"Inactive"</span> }.into_view() }}
                        </td>
                    </tr>
                }).collect_view()}
            </tbody>
        </Table>
    }
}
