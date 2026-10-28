use crate::pages::index::IndexPage;
use crate::pages::not_found::NotFoundPage;
use leptos::prelude::*;
use leptos_router::components::Redirect;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base="/portfolio">
            <Routes fallback=|| view! { <NotFoundPage/> }>
                <Route path=path!("/") view=|| view! { <Redirect path="/index.php" /> } />
                <Route path=path!("/index.php") view=IndexPage/>
            </Routes>
        </Router>
    }
}
