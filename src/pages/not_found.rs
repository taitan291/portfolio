use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="min-h-[50vh] flex flex-col items-center justify-center gap-5 p-6 text-center">
            <h1 class="text-3xl font-bold">"404 — ページが見つかりません"</h1>
            <p>"URLを確認するか、トップページへお戻りください。"</p>
            <A href="/" attr:class="text-cyber-accent underline">"トップページへ戻る"</A>
        </div>
    }
}
