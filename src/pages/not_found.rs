use leptos::prelude::*;
use leptos_router::components::A;

use crate::layouts::layout::Layout;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <Layout>
            <section class="relative isolate overflow-hidden">
                <div class="hero-grid absolute inset-0 -z-10 animate-fade-in" aria-hidden="true" />
                <div class="container-page flex min-h-[60vh] flex-col items-center justify-center gap-6 py-24 text-center">
                    <p class="animate-fade-up font-mono text-sm text-accent-text">"// error"</p>
                    <h1 class="animate-fade-up text-8xl font-bold leading-none tracking-tighter [animation-delay:80ms] sm:text-9xl">"404"</h1>
                    <p class="animate-fade-up text-lg font-medium [animation-delay:160ms]">"ページが見つかりません"</p>
                    <p class="animate-fade-up text-muted [animation-delay:220ms]">"URLを確認するか、トップページへお戻りください。"</p>
                    <A href="/" attr:class="group btn-primary mt-4 animate-fade-up [animation-delay:300ms]">
                        <span aria-hidden="true" class="transition-transform duration-300 group-hover:-translate-x-1">"←"</span>
                        "トップページへ戻る"
                    </A>
                </div>
            </section>
        </Layout>
    }
}
