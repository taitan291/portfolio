use crate::components::element::footer::Footer;
use crate::components::element::header::Header;
use leptos::prelude::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="page-shell mx-auto w-full max-w-4xl bg-cyber-layout lg:flex lg:min-h-dvh lg:max-w-none lg:flex-col">
            <Header />
            <main id="main-content" tabindex="-1" class="lg:flex-1">
                {children()}
            </main>
            <Footer />
        </div>
    }
}
