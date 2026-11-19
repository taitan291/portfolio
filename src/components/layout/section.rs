use leptos::prelude::*;

/// 番号付きのラベル（例: `01 / about`）と大きな見出しを持つページ内セクション。
/// `tag` はヘッダーのリンク先になる ID。
#[component]
pub fn Section(
    #[prop(into)] title: String,
    #[prop(into)] tag: String,
    /// ラベルに表示する 2 桁の番号。
    #[prop(into)]
    number: String,
    /// 見出しの横に表示する短い説明。
    #[prop(optional, into)]
    lead: String,
    /// 見出しの横に置く要素（ドロップダウンなど）。
    #[prop(optional, into)]
    actions: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    let label = tag.clone();
    view! {
        <section id=tag class="py-20 sm:py-28">
            <div class="container-page">
                <div data-reveal="" class="mb-10 flex flex-col gap-4 sm:mb-14 md:flex-row md:items-end md:justify-between">
                    <div>
                        <p class="font-mono text-sm text-accent-text">
                            {number}<span class="mx-2 text-muted">"/"</span>{label}
                        </p>
                        <h2 class="mt-3 text-4xl font-bold tracking-tight sm:text-5xl">{title}</h2>
                    </div>
                    {(!lead.is_empty() || actions.is_some()).then(|| view! {
                        <div class="flex flex-col gap-4 md:items-end">
                            {(!lead.is_empty()).then(|| view! {
                                <p class="max-w-sm text-sm text-muted md:text-right">{lead}</p>
                            })}
                            {actions.map(|actions| actions.run())}
                        </div>
                    })}
                </div>
                {children()}
            </div>
        </section>
    }
}
