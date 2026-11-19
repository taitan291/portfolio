use leptos::prelude::*;
use leptos_router::components::A;

use crate::layouts::layout::Layout;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <Layout>
            <section class="relative isolate overflow-hidden">
                <div class="hero-grid absolute inset-0 -z-10" aria-hidden="true" />
                <div class="container-page flex min-h-[60vh] flex-col items-center justify-center gap-6 py-24 text-center">
                    <p class="font-mono text-sm text-accent-text">"// error"</p>
                    <h1 class="text-8xl font-bold leading-none tracking-tighter sm:text-9xl">"404"</h1>
                    <p class="text-lg font-medium">"ページが見つかりません"</p>
                    <p class="text-muted">"URLを確認するか、トップページへお戻りください。"</p>
                    <A href="/" attr:class="btn-primary mt-4">
                        <span aria-hidden="true">"←"</span>
                        "トップページへ戻る"
                    </A>
                </div>
            </section>
        </Layout>
    }
}
