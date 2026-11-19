use leptos::prelude::*;

/// Page section with a numbered eyebrow label (e.g. `01 / About`) and a large heading.
#[component]
pub fn Section(
    #[prop(into)] title: String,
    #[prop(into)] tag: String,
    /// Two-digit index shown in the eyebrow label.
    #[prop(into)]
    number: String,
    /// Short description shown next to the heading.
    #[prop(optional, into)]
    lead: String,
    children: Children,
) -> impl IntoView {
    let label = tag.clone();
    view! {
        <section id=tag class="scroll-mt-20 py-20 sm:py-28">
            <div class="container-page">
                <div class="mb-10 flex flex-col gap-4 sm:mb-14 md:flex-row md:items-end md:justify-between">
                    <div>
                        <p class="font-mono text-sm text-accent-text">
                            {number}<span class="mx-2 text-muted">"/"</span>{label}
                        </p>
                        <h2 class="mt-3 text-4xl font-bold tracking-tight sm:text-5xl">{title}</h2>
                    </div>
                    {(!lead.is_empty()).then(|| view! {
                        <p class="max-w-sm text-sm text-muted md:text-right">{lead}</p>
                    })}
                </div>
                {children()}
            </div>
        </section>
    }
}
