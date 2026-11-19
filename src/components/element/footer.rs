use leptos::prelude::*;

use super::header::CONTACT_ID;

pub const GITHUB_URL: &str = "https://github.com/taitan291";

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer
            id=CONTACT_ID
            class="border-t border-line bg-surface"
            style="padding-bottom: env(safe-area-inset-bottom, 0px)"
        >
            <div data-reveal="" class="container-page py-20 sm:py-28">
                <p class="font-mono text-sm text-accent-text">
                    "04"<span class="mx-2 text-muted">"/"</span>{CONTACT_ID}
                </p>
                <h2 class="mt-3 text-4xl font-bold tracking-tight sm:text-6xl">
                    "Let's connect"<span class="text-accent-text">"."</span>
                </h2>
                <p class="mt-5 max-w-xl text-muted">
                    "制作物のソースコードや日々の活動は GitHub で公開しています。気軽にのぞいてみてください。"
                </p>
                <div class="mt-8 flex flex-wrap gap-3">
                    <a href=GITHUB_URL target="_blank" rel="noopener noreferrer" class="group btn-primary">
                        "GitHub"
                        <span aria-hidden="true" class="transition-transform duration-300 group-hover:-translate-y-0.5 group-hover:translate-x-0.5">"↗"</span>
                    </a>
                    <a href="#main-content" class="group btn-ghost">
                        "ページトップへ"
                        <span aria-hidden="true" class="transition-transform duration-300 group-hover:-translate-y-1">"↑"</span>
                    </a>
                </div>
            </div>
            <div class="border-t border-line">
                <div class="container-page flex flex-col gap-2 py-6 font-mono text-xs text-muted sm:flex-row sm:items-center sm:justify-between">
                    <p>"© 2026 taitan291 All Rights Reserved."</p>
                    <p>"Built with Rust + Leptos"</p>
                </div>
            </div>
        </footer>
    }
}
