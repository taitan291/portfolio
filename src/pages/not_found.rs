use leptos::prelude::*;

use crate::components::game::ShootingGallery;
use crate::data::site::INDEX_URL;

/// 404 ページ。存在しない URL では、射的ゲームを全画面で表示する。
#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! { <ShootingGallery home_href=INDEX_URL /> }
}
