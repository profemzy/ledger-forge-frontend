use leptos::*;
use leptos::ev;
use leptos_router::A;
use crate::utils::format::mask_money_input;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

fn button_classes(variant: &str, size: &str, disabled: bool) -> String {
    let base = "inline-flex items-center justify-center rounded transition shadow-sm disabled:opacity-50 disabled:cursor-not-allowed";
    let v = match variant {
        "secondary" => "bg-gray-100 text-gray-800 hover:bg-gray-200 border border-gray-200",
        "danger" => "bg-red-600 text-white hover:bg-red-700",
        "ghost" => "bg-transparent text-gray-700 hover:bg-gray-100",
        _ => "bg-blue-600 text-white hover:bg-blue-700",
    };
    let s = match size {
        "sm" => "text-sm px-3 py-1.5",
        _ => "text-sm px-4 py-2",
    };
    let extra = if disabled { " opacity-50" } else { "" };
    format!("{} {} {}{}", base, v, s, extra)
}

#[component]
pub fn Button(
    #[prop(optional, into)] variant: Option<String>,
    #[prop(optional, into)] size: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] r#type: Option<&'static str>,
    #[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let v = variant.unwrap_or_else(|| "primary".into());
    let s = size.unwrap_or_else(|| "md".into());
    let classes = button_classes(&v, &s, disabled);
    view! {
        <button class=classes disabled=disabled r#type={r#type.unwrap_or("button")} on:click=move |e| { if let Some(cb) = on_click.as_ref() { cb.call(e); } }>
            {children()}
        </button>
    }
}

#[component]
pub fn ButtonLink(
    #[prop(into)] href: String,
    #[prop(optional, into)] variant: Option<String>,
    #[prop(optional, into)] size: Option<String>,
    children: Children,
) -> impl IntoView {
    let v = variant.unwrap_or_else(|| "primary".into());
    let s = size.unwrap_or_else(|| "md".into());
    let classes = button_classes(&v, &s, false);
    view! { <A href=href class=classes>{children()}</A> }
}

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! { <div class="bg-white dark:bg-gray-900 rounded shadow p-4">{children()}</div> }
}

#[component]
pub fn CardHeader(children: Children) -> impl IntoView {
    view! { <div class="mb-2 flex items-center justify-between">{children()}</div> }
}

#[component]
pub fn CardTitle(children: Children) -> impl IntoView {
    view! { <div class="text-lg font-semibold">{children()}</div> }
}

#[component]
pub fn CardContent(children: Children) -> impl IntoView {
    view! { <div class="">{children()}</div> }
}

#[component]
pub fn TextInput(
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] inputmode: Option<&'static str>,
) -> impl IntoView {
    let cls = class.unwrap_or_else(|| "w-full border rounded px-3 py-2".into());
    view! {
        <input class=cls type="text" placeholder=placeholder.unwrap_or_default()
            prop:value=move || value.get()
            on:input=move |e| set_value.set(event_target_value(&e)) inputmode=inputmode.unwrap_or("text") />
    }
}

#[component]
pub fn MoneyInput(
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let cls = class.unwrap_or_else(|| "w-full border rounded px-3 py-2".into());
    view! {
        <input class=cls type="text" inputmode="decimal" placeholder=placeholder.unwrap_or_else(|| "0.00".into())
            prop:value=move || value.get()
            on:input=move |e| {
                let s = event_target_value(&e);
                set_value.set(mask_money_input(&s));
            }
        />
    }
}

#[component]
pub fn Table(children: Children) -> impl IntoView {
    view! { <div class="bg-white dark:bg-gray-900 rounded shadow overflow-hidden"><table class="w-full border-collapse">{children()}</table></div> }
}

#[derive(Clone, Debug)]
pub struct ChartPoint { pub label: String, pub amount: Decimal }

#[component]
pub fn ChartBars(
    entries: Vec<ChartPoint>,
    #[prop(optional)] positive_color: Option<&'static str>,
    #[prop(optional)] negative_color: Option<&'static str>,
) -> impl IntoView {
    let pos = positive_color.unwrap_or("bg-green-500");
    let neg = negative_color.unwrap_or("bg-red-500");
    let max_abs = entries.iter().map(|p| p.amount.abs()).fold(Decimal::ZERO, |acc, x| if x > acc { x } else { acc });
    let max_f = max_abs.to_f64().unwrap_or(1.0).max(0.01);
    view! {
        <div class="overflow-x-auto">
            <div class="min-w-max flex items-end gap-3 h-40 pr-2">
                {entries.into_iter().map(|pt| {
                    let height_px = (pt.amount.abs().to_f64().unwrap_or(0.0) / max_f * 120.0).max(4.0);
                    let bar_class = if pt.amount >= Decimal::ZERO { pos } else { neg };
                    let tooltip = format!("{}: {}", pt.label.clone(), crate::utils::format::format_money(&pt.amount));
                    view!{
                        <div class="flex flex-col items-center gap-1 w-14" title=tooltip.clone()>
                            <div class="w-full bg-gray-200 dark:bg-gray-800 rounded-sm h-32 flex items-end" title=tooltip.clone()>
                                <div class=bar_class style=format!("height: {:.1}px; width: 100%;", height_px) title=tooltip.clone()></div>
                            </div>
                            <div class="text-xs text-gray-600 dark:text-gray-300">{pt.label.clone()}</div>
                            <div class="text-xs font-mono hidden md:block">{crate::utils::format::format_money_compact(&pt.amount)}</div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
