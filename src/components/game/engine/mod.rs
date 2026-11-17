//! ゲームロジック。web-sys に依存しないので、ネイティブの `cargo test` で検証できる。

pub mod config;
pub mod effects;
pub mod hit;
pub mod layout;
pub mod ranking;
pub mod rng;
pub mod sequence;
pub mod spawner;
pub mod state;
pub mod target;
