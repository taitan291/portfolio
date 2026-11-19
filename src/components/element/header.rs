use leptos::prelude::*;
use leptos_router::components::A;

use crate::data::site::{GITHUB_URL, INDEX_PATH, PAGE_LINKS};

/// サイト共通のヘッダー（スキップリンク・ナビゲーション・GitHub リンク）。
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header
            class="sticky top-0 z-40 border-b border-line/70 bg-base/75 backdrop-blur-lg"
            style="padding-top: env(safe-area-inset-top, 0px)"
        >
            <a
                href="#main-content"
                class="sr-only focus:not-sr-only focus:absolute focus:left-4 focus:top-4 focus:z-50 focus:rounded-full focus:bg-accent focus:px-4 focus:py-2 focus:text-accent-on"
            >
                "本文へ移動"
            </a>
            <nav
                aria-label="メインナビゲーション"
                class="container-page flex flex-wrap items-center justify-between gap-x-4"
            >
                <A href=INDEX_PATH attr:class="group inline-flex min-h-14 items-center gap-2 font-mono text-sm font-medium">
                    <span class="grid h-7 w-7 place-items-center rounded-md bg-accent text-xs font-bold text-accent-on transition-transform group-hover:rotate-6">
                        "t"
                    </span>
                    "taitan291"
                </A>
                // 560px 未満ではリンクを 2 段目に回す。ごく狭い画面では番号を隠し、1 行に収める。
                <div class="-mx-2.5 flex w-full items-center pb-2 min-[560px]:mx-0 min-[560px]:w-auto min-[560px]:pb-0">
                    <ul class="flex flex-1 flex-wrap font-mono text-xs">
                        {PAGE_LINKS.into_iter().enumerate().map(|(index, (label, id))| view! {
                            <li>
                                <A
                                    href=format!("{INDEX_PATH}#{id}")
                                    attr:class="group inline-flex min-h-11 min-w-11 items-center gap-1.5 rounded-full px-2.5 text-muted transition-colors hover:text-fg sm:px-3"
                                >
                                    <span class="hidden text-accent-text min-[400px]:inline">{format!("{:02}", index + 1)}</span>
                                    <span class="nav-label">{label}</span>
                                </A>
                            </li>
                        }).collect_view()}
                    </ul>
                    <a
                        href=GITHUB_URL
                        target="_blank"
                        rel="noopener noreferrer"
                        aria-label="GitHub(新しいタブで開く)"
                        class="inline-flex min-h-11 min-w-11 items-center justify-center rounded-full text-muted transition-colors hover:text-fg"
                    >
                        <GithubIcon class="h-5 w-5" />
                    </a>
                </div>
            </nav>
            <div class="scroll-progress" aria-hidden="true" />
        </header>
    }
}

/// GitHub のマーク。色は `currentColor` を継承する。
#[component]
fn GithubIcon(#[prop(into)] class: String) -> impl IntoView {
    view! {
        <svg aria-hidden="true" viewBox="0 0 16 16" class=format!("fill-current {class}")>
            <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27s1.36.09 2 .27c1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8Z"/>
        </svg>
    }
}
