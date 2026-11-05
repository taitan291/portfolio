use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-site-border bg-site-surface py-6 text-center text-xs text-site-muted">
            <p>"© 2026 taitan291 All Rights Reserved."</p>
        </footer>
    }
}
