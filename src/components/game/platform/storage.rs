//! 個人ランキングの `localStorage` 読み書き。
//!
//! プライベートモードなどで使えない場合は、何もせず空のランキングとして扱う。

use crate::components::game::engine::ranking::Ranking;
use web_sys::Storage;

const KEY: &str = "portfolio.404-game.ranking.v1";

fn storage() -> Option<Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

pub fn load_ranking() -> Ranking {
    storage()
        .and_then(|s| s.get_item(KEY).ok().flatten())
        .map(|s| Ranking::parse(&s))
        .unwrap_or_default()
}

pub fn save_ranking(ranking: &Ranking) {
    if let Some(s) = storage() {
        let _ = s.set_item(KEY, &ranking.serialize());
    }
}

pub fn clear_ranking() {
    if let Some(s) = storage() {
        let _ = s.remove_item(KEY);
    }
}
