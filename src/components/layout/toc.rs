use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Toc(#[prop(into)] title: String, #[prop(into)] tag: String) -> impl IntoView {
    view! {
        <A href=tag attr:class="block rounded-lg border border-site-border bg-site-surface px-5 py-4 text-sm font-medium transition-colors hover:border-site-border-strong hover:bg-site-subtle">
            {title}
        </A>
    }
}
