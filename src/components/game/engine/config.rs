//! チューニング用の設定値（設計書 18 章）。時間はすべて秒、座標は論理 px。
//! シーケンスの構造（長さ・パートの区切り）は `sequence` に置く。

/// 誤射 1 回あたりのペナルティ。
pub const PENALTY_SEC: f64 = 0.5;

/// 誤射ペナルティの表示（`+0.5` など）。HUD・ポップアップ・説明文で共有する。
pub fn penalty_label() -> String {
    format!("+{PENALTY_SEC:.1}")
}

/// 空振り 1 回あたりのペナルティ。
pub const MISS_SHOT_PENALTY_SEC: f64 = 0.0;

/// 同時表示数の下限・上限。
pub const VISIBLE_MIN: usize = 2;
pub const VISIBLE_MAX: usize = 5;
/// 目標同時表示数の引き直し間隔。
pub const DESIRED_REROLL_SEC: f64 = 1.5;
/// 開始直後のウォームアップ時間と、その間の同時表示数の上限。
pub const WARMUP_SEC: f64 = 2.0;
pub const WARMUP_MAX: usize = 3;

/// 的が出ている（`Up`）時間。
pub const UP_MIN_SEC: f64 = 0.8;
pub const UP_MAX_SEC: f64 = 1.5;
/// 正解の的の `Up` 時間の下限。
pub const TARGET_MIN_UP_SEC: f64 = 0.8;
/// せり上がり・引っ込みの時間。
pub const RISE_SEC: f64 = 0.10;
pub const SINK_SEC: f64 = 0.10;
/// 撃たれた的の破壊演出の時間。
pub const HIT_ANIM_SEC: f64 = 0.25;

/// スポーン間隔。
pub const SPAWN_GAP_MIN: f64 = 0.12;
pub const SPAWN_GAP_MAX: f64 = 0.35;
/// 正解が画面に出ていない最大時間。
pub const TARGET_GAP_MAX: f64 = 0.6;
/// 正解が画面にないとき、スポーンで正解を選ぶ確率。
pub const TARGET_SPAWN_PROB: f64 = 0.60;
/// 外れの的のうち、紛らわし（シーケンス内の文字）を選ぶ確率。残りはダミー。
pub const SEQ_DECOY_RATIO: f64 = 0.75;
/// 同じスロットに再出現できるまでの時間。
pub const SLOT_COOLDOWN_SEC: f64 = 0.15;

/// `NOT FOUND` パートで同時表示数に足す数（この範囲の乱数）。
pub const NOTFOUND_EXTRA_MIN: usize = 1;
pub const NOTFOUND_EXTRA_MAX: usize = 3;
/// 同時表示数の絶対的な上限。
pub const VISIBLE_LIMIT: usize = VISIBLE_MAX + NOTFOUND_EXTRA_MAX;
/// `NOT FOUND` パートでは正解を出やすくする。
pub const NOTFOUND_TARGET_GAP_MAX: f64 = 0.3;
pub const NOTFOUND_TARGET_SPAWN_PROB: f64 = 0.85;

/// ジッター（scale 1.0 のとき）。
pub const JITTER_X: f64 = 6.0;
pub const JITTER_Y: f64 = 3.0;
/// 隣の的との重なりの許容割合（的の幅に対する比）。
pub const MAX_OVERLAP_RATIO: f64 = 0.15;

/// 当たり半径（scale 1.0）。
pub const HIT_RADIUS: f64 = 14.0;
/// タッチ時の当たり半径の加算。
pub const TOUCH_HIT_BONUS: f64 = 4.0;
/// 同一ポインタの連射制限（ms）。
pub const FIRE_COOLDOWN_MS: f64 = 60.0;

/// カウントダウン 1 段の長さと、`GO!` の表示時間。
pub const COUNTDOWN_STEP_SEC: f64 = 0.7;
pub const GO_SEC: f64 = 0.4;
/// クリア演出からリザルト表示までの時間。
pub const CLEAR_DELAY_SEC: f64 = 0.8;
/// HUD のタイム更新間隔（ms）。
pub const HUD_UPDATE_MS: f64 = 100.0;
/// パーティクルの上限。
pub const MAX_PARTICLES: usize = 100;
/// 「NEXT: x」の大表示を出すか。
pub const SHOW_NEXT_GLYPH: bool = false;

/// 的の寸法（scale 1.0）。板は直径 `PLATE_D` の円、その下に支柱が付く。
pub const TARGET_H: f64 = 40.0;
pub const PLATE_D: f64 = 28.0;

/// タイトル画面のデモで同時に出す的の数と、出ている時間。
pub const DEMO_VISIBLE: usize = 3;
pub const DEMO_UP_MIN_SEC: f64 = 1.2;
pub const DEMO_UP_MAX_SEC: f64 = 2.4;
/// 1 フレームの dt の上限。
pub const MAX_DT: f64 = 0.05;
