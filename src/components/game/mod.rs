//! 404 ページで遊べる射的ゲーム。
//!
//! - `engine`: ゲームロジック（純 Rust・web-sys 非依存。`cargo test` 可）
//! - `platform`: Canvas 描画・Web Audio・入力・rAF ループ（web-sys 依存）
//! - `component` / `ui`: Leptos コンポーネント（Canvas + DOM オーバーレイ）

mod component;
mod engine;
mod platform;
mod ui;

pub use component::ShootingGallery;
