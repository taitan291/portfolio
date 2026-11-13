use leptos::prelude::*;
use leptos_router::components::A;

/// `/` に置くイースターエッグ。Apache の初期ページ風の見た目で、
/// 「index.php なのに中身は Rust」という小ネタを仕込んでいる。
#[component]
pub fn RootPage() -> impl IntoView {
    let (revealed, set_revealed) = signal(false);

    view! {
            <div class="mx-auto flex min-h-dvh max-w-2xl flex-col justify-center gap-6 px-4 py-12">
                <h1 class="border-b border-site-border pb-3 text-3xl font-bold text-site-accent">
                    "It works!"
                </h1>
                <p>
                    "This is the default welcome page used to test the correct operation of the web server after installation."
                </p>
                <p class="text-site-subtext">
                    "If you can read this page, it means that the web server installed at this site is working properly, but has not yet been configured."
                </p>

                <div class="rounded border border-site-border bg-site-subtle p-4 text-sm">
                    <p class="font-semibold">"Not looking for this page?"</p>
                    <p class="mt-1 text-site-subtext">
                        "ここは "
                        <button
                            type="button"
                            class="rounded px-1 underline decoration-dotted underline-offset-4 hover:text-site-accent"
                            aria-expanded=move || revealed.get().to_string()
                            on:click=move |_| set_revealed.update(|v| *v = !*v)
                        >
                            "web server"
                        </button>
                        " のトップです。探しているのはたぶんこちら:"
                    </p>
                    <p class="mt-2">
                        <A href="/index.php" attr:class="text-site-link underline underline-offset-4">
                            "/index.php"
                        </A>
                    </p>
                </div>

                <Show when=move || revealed.get()>
                    <pre
                        role="note"
                        class="overflow-x-auto rounded border border-site-border bg-site-subtle p-4 text-xs leading-relaxed text-site-subtext"
                    >
    {"$ file index.php
index.php: not PHP script, it's actually Rust (compiled to WASM) 🦀

<?php echo \"嘘です\"; ?>
// ↑ このサイトに PHP は一行もありません。"}
                    </pre>
                </Show>

                <p class="text-xs text-site-muted">
                    "Apache/2.4.58 (Rust) Server at taitan291.github.io Port 443"
                </p>
            </div>
        }
}
