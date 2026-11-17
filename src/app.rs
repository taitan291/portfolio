use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::pages::index::IndexPage;
use crate::pages::not_found::NotFoundPage;
use crate::pages::root::RootPage;

/// ルーティング。パスは `data::site::INDEX_PATH` と揃える（`path!` はリテラルしか受け付けない）。
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base="/portfolio">
            <Routes fallback=|| view! { <NotFoundPage/> }>
                <Route path=path!("/") view=RootPage/>
                <Route path=path!("/index.php") view=IndexPage/>
            </Routes>
        </Router>
    }
}
