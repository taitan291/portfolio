use crate::components::element::footer::Footer;
use crate::components::element::header::Header;
use leptos::prelude::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="flex min-h-dvh flex-col">
            <Header />
            <main id="main-content" tabindex="-1" class="flex-1 scroll-mt-20 focus:outline-none">
                {children()}
            </main>
            <Footer />
        </div>
    }
}
