use leptos::children::Children;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::*;

#[component]
pub fn OshiBlock(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 p-5 m-2 bg-surface rounded-xl border border-line">
            <h2 class="text-fg">{children()}</h2>
        </div>
    }
}

#[component]
pub fn OshiItem(#[prop(into)] label: String, children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1">
            <span class="text-xs font-semibold text-muted uppercase tracking-wider">
                {label}
            </span>
            <div class="text-sm text-fg leading-relaxed whitespace-pre-line">
                {children()}
            </div>
        </div>
    }
}
