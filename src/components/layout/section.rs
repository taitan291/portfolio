use leptos::prelude::*;

#[component]
pub fn Section(
    #[prop(into)] title: String,
    #[prop(into)] tag: String,
    /// タイトルの横に置く要素(ドロップダウンなど)
    #[prop(optional, into)]
    actions: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    view! {
        <section
            id=tag
            class="border-t border-site-border py-8 text-site-text sm:py-10 scroll-mt-6"
        >
            <div class="mb-5 flex flex-wrap items-center gap-x-4 gap-y-2">
                <h2 class="text-xl font-semibold tracking-tight text-site-accent">{title}</h2>
                {actions.map(|a| a.run())}
            </div>
            <div>{children()}</div>
        </section>
    }
}
