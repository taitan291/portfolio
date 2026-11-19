use leptos::prelude::*;
use leptos_router::components::A;

/// Section ids shared by the header navigation and the page.
pub const ABOUT_ID: &str = "about";
pub const SKILLS_ID: &str = "skills";
pub const WORKS_ID: &str = "works";
pub const CONTACT_ID: &str = "contact";
pub const PAGE_LINKS: [(&str, &str); 4] = [
    ("About", ABOUT_ID),
    ("Skills", SKILLS_ID),
    ("Works", WORKS_ID),
    ("Contact", CONTACT_ID),
];

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
                <A href="/" attr:class="group inline-flex min-h-14 items-center gap-2 font-mono text-sm font-medium">
                    <span class="grid h-7 w-7 place-items-center rounded-md bg-accent text-xs font-bold text-accent-on transition-transform group-hover:rotate-6">
                        "t"
                    </span>
                    "taitan291"
                </A>
                // Below 560px the links get their own row; the numbers are dropped on very
                // narrow screens so all four links fit on that single row.
                <ul class="-mx-2.5 flex w-full flex-wrap pb-2 font-mono text-xs min-[560px]:mx-0 min-[560px]:w-auto min-[560px]:pb-0">
                    {PAGE_LINKS.into_iter().enumerate().map(|(index, (label, id))| view! {
                        <li>
                            <A
                                href=format!("/#{id}")
                                attr:class="group inline-flex min-h-11 min-w-11 items-center gap-1.5 rounded-full px-2.5 text-muted transition-colors hover:text-fg sm:px-3"
                            >
                                <span class="hidden text-accent-text min-[400px]:inline">{format!("{:02}", index + 1)}</span>
                                <span class="nav-label">{label}</span>
                            </A>
                        </li>
                    }).collect_view()}
                </ul>
            </nav>
            <div class="scroll-progress" aria-hidden="true" />
        </header>
    }
}
