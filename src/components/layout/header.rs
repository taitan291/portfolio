use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::ui::github_icon::GithubIcon;
use crate::data::site::{GITHUB_URL, PAGE_LINKS};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="border-b border-site-border bg-site-surface text-site-subtext">
            <a href="#main-content" class="sr-only focus:not-sr-only focus:block focus:p-4">
                "本文へ移動"
            </a>
            <nav aria-label="メインナビゲーション" class="py-3 sm:py-4 flex flex-wrap justify-between items-center gap-x-4 gap-y-1 text-sm">
                <A href="/" attr:class="inline-flex min-h-11 items-center font-semibold tracking-tight text-site-text">"taitan's Portfolio"</A>
                <div class="flex w-full items-center gap-2 min-[480px]:w-auto">
                <ul class="flex flex-wrap gap-2">
                    {PAGE_LINKS.into_iter().map(|(label, id)| view! {
                        <li><A href=format!("/#{id}") attr:class="inline-flex min-h-11 min-w-11 items-center justify-center rounded px-3 transition-colors hover:text-site-accent hover:underline underline-offset-4">{label}</A></li>
                    }).collect_view()}
                </ul>
                <a
                    href=GITHUB_URL
                    target="_blank"
                    rel="noopener noreferrer"
                    aria-label="GitHub(新しいタブで開く)"
                    class="inline-flex min-h-11 min-w-11 items-center justify-center rounded transition-colors hover:text-site-accent"
                >
                    <GithubIcon class="h-6 w-6" />
                </a>
                </div>
            </nav>
        </header>
    }
}
