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

pub const GITHUB_URL: &str = "https://github.com/taitan291";

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="border-b border-cyber-border bg-cyber-surface text-cyber-layouttext">
            <a href="#main-content" class="sr-only focus:not-sr-only focus:block focus:p-4">
                "本文へ移動"
            </a>
            <nav aria-label="メインナビゲーション" class="py-3 sm:py-4 flex flex-wrap justify-between items-center gap-x-4 gap-y-1 text-sm">
                <A href="/" attr:class="inline-flex min-h-11 items-center font-semibold tracking-tight text-cyber-text">"taitan's Portfolio"</A>
                <div class="flex w-full items-center gap-2 min-[480px]:w-auto">
                <ul class="flex flex-wrap gap-2">
                    {PAGE_LINKS.into_iter().map(|(label, id)| view! {
                        <li><A href=format!("/#{id}") attr:class="inline-flex min-h-11 min-w-11 items-center justify-center rounded px-3 transition-colors hover:text-cyber-accent hover:underline underline-offset-4">{label}</A></li>
                    }).collect_view()}
                </ul>
                <a
                    href=GITHUB_URL
                    target="_blank"
                    rel="noopener noreferrer"
                    aria-label="GitHub(新しいタブで開く)"
                    class="inline-flex min-h-11 min-w-11 items-center justify-center rounded transition-colors hover:text-cyber-accent"
                >
                    <svg aria-hidden="true" viewBox="0 0 16 16" class="h-6 w-6 fill-current">
                        <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27s1.36.09 2 .27c1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8Z"/>
                    </svg>
                </a>
                </div>
            </nav>
        </header>
    }
}
