use crate::pages::index::IndexPage;
use crate::pages::not_found::NotFoundPage;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base="/portfolio">
            <Routes fallback=|| view! { <NotFoundPage/> }>
                <Route path=path!("/") view=IndexPage/>
                <Route path=path!("/404") view=NotFoundPage/>
            </Routes>
        </Router>
    }
}
