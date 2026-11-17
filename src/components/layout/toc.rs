use leptos::prelude::*;
use leptos_router::components::A;

/// 目次の 1 項目。
#[component]
pub fn Toc(#[prop(into)] title: String, #[prop(into)] href: String) -> impl IntoView {
    view! {
        <A href=href attr:class="block rounded-lg border border-site-border bg-site-surface px-5 py-4 text-sm font-medium transition-colors hover:border-site-border-strong hover:bg-site-subtle">
            {title}
        </A>
    }
}
