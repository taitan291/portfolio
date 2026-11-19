use leptos::prelude::*;
use leptos_router::components::A;

use crate::data::site::INDEX_URL;

/// トップ（`/`）。Web サーバーの初期ページ風のネタページで、本編へのリンクを置く。
#[component]
pub fn RootPage() -> impl IntoView {
    let (revealed, set_revealed) = signal(false);

    view! {
            <div class="mx-auto flex min-h-dvh max-w-2xl flex-col justify-center gap-6 px-4 py-12">
                <h1 class="border-b border-line pb-3 text-3xl font-bold text-fg">
                    "It works!"
                </h1>
                <p>
                    "This is the default welcome page used to test the correct operation of the web server after installation."
                </p>
                <p class="text-muted">
                    "If you can read this page, it means that the web server installed at this site is working properly, but has not yet been configured."
                </p>

                <div class="rounded border border-line bg-surface-2 p-4 text-sm">
                    <p class="font-semibold">"Not looking for this page?"</p>
                    <p class="mt-1 text-muted">
                        "ここは "
                        <button
                            type="button"
                            class="rounded px-1 underline decoration-dotted underline-offset-4 hover:text-accent-text"
                            aria-expanded=move || revealed.get().to_string()
                            on:click=move |_| set_revealed.update(|v| *v = !*v)
                        >
                            "web server"
                        </button>
                        " のトップです。探しているのはたぶん:"
                    </p>
                    <p class="mt-2">
                        <A href=INDEX_URL attr:class="text-accent-text underline underline-offset-4">
                            "こちら"
                        </A>
                    </p>
                </div>

                <Show when=move || revealed.get()>
                    <pre
                        role="note"
                        class="overflow-x-auto rounded border border-line bg-surface-2 p-4 text-xs leading-relaxed text-muted"
                    >
    {"$ file index.php
index.php: not PHP script, it's actually Rust (compiled to WASM) 🦀

<?php echo \"嘘です\"; ?>
// ↑ このサイトに PHP は一行もありません。"}
                    </pre>
                </Show>
            </div>
        }
}
