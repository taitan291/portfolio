use leptos::prelude::*;

#[component]
pub fn Dropdown(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <details>
            <summary class="cursor-pointer px-4 py-2 text-2xl font-bold text-cyber-accent rounded-md">
                {title}
            </summary>
            <div class="px-4 indent-1">{children()}</div>
        </details>
    }
}
