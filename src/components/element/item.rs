use leptos::children::Children;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::*;

#[component]
pub fn OshiBlock(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 p-5 m-2 bg-zinc-50  rounded-xl border border-zinc-100 ">
            <h2 class="text-black">{children()}</h2>
        </div>
    }
}

#[component]
pub fn OshiItem(#[prop(into)] label: String, children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1">
            <span class="text-xs font-semibold text-cyber-muted uppercase tracking-wider">
                {label}
            </span>
            <div class="text-sm text-cyber-text leading-relaxed whitespace-pre-line">
                {children()}
            </div>
        </div>
    }
}
