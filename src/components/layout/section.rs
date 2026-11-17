use leptos::prelude::*;

/// 見出し付きのページ内セクション。`id` はヘッダー・目次のリンク先になる。
#[component]
pub fn Section(
    #[prop(into)] title: String,
    #[prop(into)] id: String,
    /// タイトルの横に置く要素(ドロップダウンなど)
    #[prop(optional, into)]
    actions: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    view! {
        <section
            id=id
            class="border-t border-site-border pt-5 pb-8 text-site-text sm:pt-6 sm:pb-10 scroll-mt-6"
        >
            <div class="mb-5 flex flex-wrap items-center gap-x-4 gap-y-2">
                <h2 class="text-xl font-semibold tracking-tight text-site-accent">{title}</h2>
                {actions.map(|a| a.run())}
            </div>
            <div>{children()}</div>
        </section>
    }
}
