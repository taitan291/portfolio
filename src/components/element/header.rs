use leptos::prelude::*;
use leptos_router::components::A;

/// Shared by the header and the table of contents.
pub const ABOUT_ID: &str = "about";
pub const SKILLS_ID: &str = "skills";
pub const WORKS_ID: &str = "works";
pub const PAGE_LINKS: [(&str, &str); 3] = [
    ("About", ABOUT_ID),
    ("Skills", SKILLS_ID),
    ("Works", WORKS_ID),
];

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="border-b border-cyber-border bg-cyber-surface text-cyber-layouttext">
            <a href="#main-content" class="sr-only focus:not-sr-only focus:block focus:p-4">
                "本文へ移動"
            </a>
            <nav aria-label="メインナビゲーション" class="py-3 sm:py-4 flex flex-wrap justify-between items-center gap-x-4 gap-y-1 text-sm">
                <A href="/" attr:class="inline-flex min-h-11 items-center font-semibold tracking-tight text-cyber-text">"TaiTan's Portfolio"</A>
                <ul class="flex w-full flex-wrap gap-2 min-[480px]:w-auto">
                    {PAGE_LINKS.into_iter().map(|(label, id)| view! {
                        <li><A href=format!("/#{id}") attr:class="inline-flex min-h-11 min-w-11 items-center justify-center rounded px-3 transition-colors hover:text-cyber-accent hover:underline underline-offset-4">{label}</A></li>
                    }).collect_view()}
                </ul>
            </nav>
        </header>
    }
}
