use leptos::prelude::*;

#[component]
pub fn Section(
    #[prop(into)] title: String,
    #[prop(into)] tag: String,
    children: Children,
) -> impl IntoView {
    view! {
        <section
            id=tag
            class="border-t border-cyber-border py-8 text-cyber-text sm:py-10 scroll-mt-6"
        >
            <h2 class="mb-5 text-xl font-semibold tracking-tight text-cyber-accent">{title}</h2>
            <div>{children()}</div>
        </section>
    }
}
