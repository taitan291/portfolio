use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-cyber-border bg-cyber-surface py-6 text-center text-xs text-cyber-muted">
            <p>{ "© 2026 taitan291 All Rights Reserved." }</p>
        </footer>
    }
}
