use crate::components::game::ShootingGallery;
use leptos::prelude::*;

/// 404 ページ。存在しない URL では、射的ゲームを全画面で表示する。
#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! { <ShootingGallery home_href="https://taitan291.github.io/portfolio/index.php" /> }
}
