use leptos::*;

#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum ToastKind { Info, Success, Error }

#[derive(Clone)]
pub struct Toast { pub id: u64, pub message: String, pub kind: ToastKind }

#[derive(Clone)]
pub struct ToastContext {
    pub toasts: ReadSignal<Vec<Toast>>,
    pub set_toasts: WriteSignal<Vec<Toast>>,
    next_id: ReadSignal<u64>,
    set_next_id: WriteSignal<u64>,
}

impl ToastContext {
    pub fn provide() -> Self {
        let (toasts, set_toasts) = create_signal(Vec::<Toast>::new());
        let (next_id, set_next_id) = create_signal(1u64);
        let ctx = Self { toasts, set_toasts, next_id, set_next_id };
        provide_context(ctx.clone());
        ctx
    }
    pub fn push(&self, message: impl Into<String>, kind: ToastKind) {
        let id = self.next_id.get();
        self.set_next_id.set(id + 1);
        self.set_toasts.update(|v| v.push(Toast { id, message: message.into(), kind }));
        // Auto-remove after a delay
        let set_toasts = self.set_toasts;
        leptos::spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(3000).await;
            set_toasts.update(|v| v.retain(|t| t.id != id));
        });
    }
}

#[component]
pub fn Toaster() -> impl IntoView {
    let ctx = use_context::<ToastContext>().expect("ToastContext not found");
    view! {
        <div class="fixed top-3 right-3 space-y-2 z-50">
            {move || ctx.toasts.get().into_iter().map(|t| {
                let (bg, text, border) = match t.kind { ToastKind::Info => ("bg-blue-100", "text-blue-800", "border-blue-200"), ToastKind::Success => ("bg-green-100", "text-green-800", "border-green-200"), ToastKind::Error => ("bg-red-100", "text-red-800", "border-red-200") };
                view!{ <div class=format!("{} {} border px-3 py-2 rounded shadow", bg, border)><span class=format!("{}", text)>{t.message}</span></div> }
            }).collect_view()}
        </div>
    }
}
