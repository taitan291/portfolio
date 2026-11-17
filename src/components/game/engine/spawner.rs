//! 的の出現スケジューラ（設計書 7.3）。

use super::config::{
    DEMO_UP_MAX_SEC, DEMO_UP_MIN_SEC, DEMO_VISIBLE, DESIRED_REROLL_SEC, JITTER_Y,
    NOTFOUND_EXTRA_MAX, NOTFOUND_EXTRA_MIN, NOTFOUND_TARGET_GAP_MAX, NOTFOUND_TARGET_SPAWN_PROB,
    SEQ_DECOY_RATIO, SPAWN_GAP_MAX, SPAWN_GAP_MIN, TARGET_GAP_MAX, TARGET_MIN_UP_SEC,
    TARGET_SPAWN_PROB, UP_MAX_SEC, UP_MIN_SEC, VISIBLE_LIMIT, VISIBLE_MAX, VISIBLE_MIN, WARMUP_MAX,
    WARMUP_SEC,
};
use super::layout::SlotId;
use super::sequence::{Glyph, NOTFOUND_START, SEQ_DECOYS, UNRELATED_DECOYS};
use super::state::GameState;
use super::target::Target;

#[derive(Clone, Debug)]
pub struct SpawnerState {
    /// 目標同時表示数。
    pub desired: usize,
    pub desired_reroll_in: f64,
    pub next_spawn_in: f64,
    /// 正解が最後に画面に見えていたプレイ時刻。
    pub last_target_seen_at: f64,
    /// 直前に正解を出したスロット。
    pub last_target_slot: Option<SlotId>,
    /// `NOT FOUND` パートで同時表示数に足している数（それ以前は 0）。
    pub extra: usize,
    /// `NOT FOUND` パートに入ったか。
    pub in_notfound: bool,
}

impl SpawnerState {
    pub fn new() -> Self {
        Self {
            desired: VISIBLE_MIN,
            desired_reroll_in: 0.0,
            next_spawn_in: 0.0,
            // 開始直後の 1 個目は必ず正解にする。
            last_target_seen_at: -TARGET_GAP_MAX,
            last_target_slot: None,
            extra: 0,
            in_notfound: false,
        }
    }
}

impl Default for SpawnerState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    /// 表示中（撃たれた的を除く）の的の数。
    pub fn visible_count(&self) -> usize {
        self.targets.iter().filter(|t| t.is_visible()).count()
    }

    fn glyph_visible(&self, g: Glyph) -> bool {
        self.targets.iter().any(|t| t.is_visible() && t.glyph == g)
    }

    /// プレイ中のスポーン判定。毎フレーム呼ぶ。
    pub(super) fn run_spawner(&mut self, dt: f64) {
        let Some(current) = self.current_glyph() else {
            return;
        };
        let t = self.play_t;
        let notfound = self.progress >= NOTFOUND_START;
        let (target_gap_max, target_spawn_prob) = if notfound {
            (NOTFOUND_TARGET_GAP_MAX, NOTFOUND_TARGET_SPAWN_PROB)
        } else {
            (TARGET_GAP_MAX, TARGET_SPAWN_PROB)
        };

        let sp = &mut self.spawner;
        sp.desired_reroll_in -= dt;
        // NOT FOUND に入った瞬間にも引き直して、すぐ的を増やす。
        if sp.desired_reroll_in <= 0.0 || notfound != sp.in_notfound {
            sp.in_notfound = notfound;
            sp.extra = if notfound {
                self.rng.range_usize(NOTFOUND_EXTRA_MIN, NOTFOUND_EXTRA_MAX)
            } else {
                0
            };
            sp.desired = self.rng.range_usize(VISIBLE_MIN, VISIBLE_MAX) + sp.extra;
            sp.desired_reroll_in = DESIRED_REROLL_SEC;
        }
        let max_visible = (VISIBLE_MAX + sp.extra).min(VISIBLE_LIMIT);
        sp.next_spawn_in -= dt;
        let desired = if t < WARMUP_SEC {
            sp.desired.min(WARMUP_MAX)
        } else {
            sp.desired
        };

        // 最低数を割ったら即座に補充する。それ以外は 1 フレームに 1 個まで。
        let mut spawned_this_frame = false;
        loop {
            let target_present = self.glyph_visible(current);
            if target_present {
                self.spawner.last_target_seen_at = t;
            }
            let visible = self.visible_count();
            if visible >= max_visible {
                return;
            }
            let target_due =
                !target_present && t - self.spawner.last_target_seen_at >= target_gap_max;
            let below_min = visible < VISIBLE_MIN;
            let want = target_due
                || below_min
                || (!spawned_this_frame && visible < desired && self.spawner.next_spawn_in <= 0.0);
            if !want {
                return;
            }

            let spawn_target =
                !target_present && (target_due || self.rng.chance(target_spawn_prob));
            let ok = if spawn_target {
                self.spawn_glyph(current, true)
            } else if !target_present && visible + 1 >= max_visible {
                // 正解が出せるよう、最後の 1 枠は正解のために空けておく。
                false
            } else {
                match self.pick_decoy(current) {
                    Some(g) => self.spawn_glyph(g, false),
                    None => false,
                }
            };
            if !ok {
                return;
            }
            spawned_this_frame = true;
            self.spawner.next_spawn_in = self.rng.range(SPAWN_GAP_MIN, SPAWN_GAP_MAX);
        }
    }

    /// タイトル画面のデモ用。当たり判定はないので、文字は何でもよい。
    pub(super) fn run_demo_spawner(&mut self, dt: f64) {
        self.spawner.next_spawn_in -= dt;
        if self.visible_count() >= DEMO_VISIBLE || self.spawner.next_spawn_in > 0.0 {
            return;
        }
        let pool: Vec<Glyph> = SEQ_DECOYS
            .iter()
            .chain(UNRELATED_DECOYS.iter())
            .copied()
            .filter(|&g| !self.glyph_visible(g))
            .collect();
        if let Some(g) = self.rng.pick(&pool) {
            let dur = self.rng.range(DEMO_UP_MIN_SEC, DEMO_UP_MAX_SEC);
            self.spawn_with(g, false, dur);
        }
        self.spawner.next_spawn_in = self.rng.range(0.4, 0.9);
    }

    /// 外れの的の文字を選ぶ。表示中の文字と正解は選ばない。
    fn pick_decoy(&mut self, current: Glyph) -> Option<Glyph> {
        let seq: Vec<Glyph> = SEQ_DECOYS
            .iter()
            .copied()
            .filter(|&g| g != current && !self.glyph_visible(g))
            .collect();
        let unrelated: Vec<Glyph> = UNRELATED_DECOYS
            .iter()
            .copied()
            .filter(|&g| !self.glyph_visible(g))
            .collect();
        let (first, second) = if self.rng.chance(SEQ_DECOY_RATIO) {
            (seq, unrelated)
        } else {
            (unrelated, seq)
        };
        self.rng.pick(&first).or_else(|| self.rng.pick(&second))
    }

    fn spawn_glyph(&mut self, glyph: Glyph, is_target: bool) -> bool {
        let min = if is_target {
            TARGET_MIN_UP_SEC
        } else {
            UP_MIN_SEC
        };
        let dur = self.rng.range(min, UP_MAX_SEC);
        self.spawn_with(glyph, is_target, dur)
    }

    /// 空いているスロットに的を 1 個出す。空きがなければ false。
    fn spawn_with(&mut self, glyph: Glyph, is_target: bool, up_dur: f64) -> bool {
        let layout = self.layout;
        let free: Vec<SlotId> = layout
            .slots()
            .filter(|&s| self.slot_free_at[layout.slot_index(s)] <= self.clock)
            .filter(|&s| !self.targets.iter().any(|t| t.slot == s))
            .collect();
        let preferred: Vec<SlotId> = if is_target {
            free.iter()
                .copied()
                .filter(|&s| Some(s) != self.spawner.last_target_slot)
                .collect()
        } else {
            Vec::new()
        };
        let candidates = if preferred.is_empty() {
            &free
        } else {
            &preferred
        };
        let Some(slot) = self.rng.pick(candidates) else {
            return false;
        };

        let row = layout.row(slot);
        let (x, y) = layout.slot_root(slot);
        let jx = layout.max_jitter_x(row);
        let jx = self.rng.range(-jx, jx);
        let jy = self.rng.range(0.0, JITTER_Y * row.scale);
        let id = self.next_target_id;
        self.next_target_id += 1;
        self.targets.push(Target::new(
            id,
            glyph,
            slot,
            (x + jx, y - jy),
            row.y,
            row.scale,
            up_dur,
        ));
        if is_target {
            self.spawner.last_target_slot = Some(slot);
        }
        true
    }
}
