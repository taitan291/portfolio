use crate::components::element::footer::Footer;
use crate::components::element::header::Header;
use crate::components::reveal::use_reveal;
use leptos::html;
use leptos::prelude::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    let root = NodeRef::<html::Div>::new();
    use_reveal(root);

    view! {
        <div node_ref=root class="flex min-h-dvh flex-col">
            <Header />
            <main id="main-content" tabindex="-1" class="flex-1 focus:outline-none">
                {children()}
            </main>
            <Footer />
        </div>
    }
}
