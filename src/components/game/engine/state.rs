//! ゲーム全体の状態・フェーズ遷移・タイマー（設計書 4・5・9 章）。

use super::config::{
    CLEAR_DELAY_SEC, COUNTDOWN_STEP_SEC, GO_SEC, MAX_DT, MISS_SHOT_PENALTY_SEC, PENALTY_SEC,
    SEQUENCE_LEN, SLOT_COOLDOWN_SEC,
};
use super::effects::{Effects, ParticleColor, PopupKind};
use super::hit::pick_target;
use super::layout::{layout_for, Layout, Orientation};
use super::rng::Rng;
use super::sequence::{completed_set, Glyph, SEQUENCE};
use super::spawner::SpawnerState;
use super::target::Target;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    /// 404 スタート画面（デモ表示）。
    Title,
    /// 3 → 2 → 1。
    Countdown,
    /// プレイ中。
    Playing,
    /// 一時停止中。
    Paused,
    /// 最後の文字を撃ってからリザルトを出すまでの演出中。タイマーは止まっている。
    Clearing,
    /// リザルト表示。
    Result,
}

/// `performance.now()` 基準（ms）のタイマー。
#[derive(Clone, Debug, Default)]
pub struct Timer {
    pub start: Option<f64>,
    pub paused_total: f64,
    pub paused_at: Option<f64>,
    pub end: Option<f64>,
}

impl Timer {
    /// 一時停止を除いた経過時間（秒）。
    pub fn elapsed(&self, now_ms: f64) -> f64 {
        let Some(start) = self.start else {
            return 0.0;
        };
        let until = self.end.or(self.paused_at).unwrap_or(now_ms);
        ((until - start - self.paused_total) / 1000.0).max(0.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShotResult {
    /// 正解。`set` は撃ち終えた `404` のセット番号、`cleared` は全文字撃破。
    Correct { set: Option<u8>, cleared: bool },
    /// 違う的を撃った（ペナルティ）。
    Wrong,
    /// 何もない場所を撃った。
    Miss,
}

/// エンジンからプラットフォーム層への通知。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameEvent {
    CountdownTick(u8),
    Go,
    PhaseChanged(Phase),
}

/// カウントダウン表示。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CountLabel {
    Num(u8),
    Go,
}

const COUNT_FROM: u8 = 3;

pub struct GameState {
    pub phase: Phase,
    pub layout: &'static Layout,
    /// 撃ち終えた文字数（0..=17）。
    pub progress: usize,
    pub penalty_count: u32,
    pub miss_count: u32,
    pub timer: Timer,
    pub countdown_t: f64,
    /// GO からのプレイ時間（秒、一時停止を除く）。スポーナーが使う。
    pub play_t: f64,
    pub clear_t: f64,
    /// 演出用の時計（秒）。一時停止中は止まる。
    pub clock: f64,
    pub targets: Vec<Target>,
    /// スロットごとの再出現可能時刻（`clock` 基準）。
    pub slot_free_at: Vec<f64>,
    pub spawner: SpawnerState,
    pub effects: Effects,
    /// PC の照準位置。
    pub aim: Option<(f64, f64)>,
    pub rng: Rng,
    pub next_target_id: u32,
}

impl GameState {
    pub fn new(seed: u64, orientation: Orientation) -> Self {
        let layout = layout_for(orientation);
        Self {
            phase: Phase::Title,
            layout,
            progress: 0,
            penalty_count: 0,
            miss_count: 0,
            timer: Timer::default(),
            countdown_t: 0.0,
            play_t: 0.0,
            clear_t: 0.0,
            clock: 0.0,
            targets: Vec::new(),
            slot_free_at: vec![0.0; layout.slot_count()],
            spawner: SpawnerState::new(),
            effects: Effects::default(),
            aim: None,
            rng: Rng::new(seed),
            next_target_id: 0,
        }
    }

    pub fn orientation(&self) -> Orientation {
        self.layout.orientation
    }

    /// 次に撃つ文字。プレイ中以外や全文字撃破後は `None`。
    pub fn current_glyph(&self) -> Option<Glyph> {
        match self.phase {
            Phase::Playing | Phase::Paused => SEQUENCE.get(self.progress).copied(),
            _ => None,
        }
    }

    /// 一時停止を除いた実時間（秒）。
    pub fn elapsed(&self, now_ms: f64) -> f64 {
        self.timer.elapsed(now_ms)
    }

    /// ペナルティ込みの表示タイム（秒）。
    pub fn display_time(&self, now_ms: f64) -> f64 {
        self.elapsed(now_ms)
            + f64::from(self.penalty_count) * PENALTY_SEC
            + f64::from(self.miss_count) * MISS_SHOT_PENALTY_SEC
    }

    /// カウントダウン・`GO!` の表示。
    pub fn count_label(&self) -> Option<CountLabel> {
        match self.phase {
            Phase::Countdown => {
                let step = (self.countdown_t / COUNTDOWN_STEP_SEC) as u8;
                Some(CountLabel::Num(COUNT_FROM.saturating_sub(step).max(1)))
            }
            Phase::Playing if self.play_t < GO_SEC => Some(CountLabel::Go),
            _ => None,
        }
    }

    fn reset(&mut self) {
        self.progress = 0;
        self.penalty_count = 0;
        self.miss_count = 0;
        self.timer = Timer::default();
        self.countdown_t = 0.0;
        self.play_t = 0.0;
        self.clear_t = 0.0;
        self.targets.clear();
        self.slot_free_at = vec![0.0; self.layout.slot_count()];
        self.spawner = SpawnerState::new();
        self.effects.clear();
    }

    /// カウントダウンを始める（START・リトライ）。
    pub fn start_countdown(&mut self) -> Vec<GameEvent> {
        self.reset();
        self.phase = Phase::Countdown;
        vec![
            GameEvent::PhaseChanged(Phase::Countdown),
            GameEvent::CountdownTick(COUNT_FROM),
        ]
    }

    /// 404 スタート画面へ戻る。
    pub fn back_to_title(&mut self) -> Vec<GameEvent> {
        self.reset();
        self.phase = Phase::Title;
        vec![GameEvent::PhaseChanged(Phase::Title)]
    }

    /// プレイ中なら一時停止する。
    pub fn pause(&mut self, now_ms: f64) -> Vec<GameEvent> {
        if self.phase != Phase::Playing {
            return Vec::new();
        }
        self.phase = Phase::Paused;
        self.timer.paused_at = Some(now_ms);
        vec![GameEvent::PhaseChanged(Phase::Paused)]
    }

    pub fn resume(&mut self, now_ms: f64) -> Vec<GameEvent> {
        if self.phase != Phase::Paused {
            return Vec::new();
        }
        if let Some(at) = self.timer.paused_at.take() {
            self.timer.paused_total += (now_ms - at).max(0.0);
        }
        self.phase = Phase::Playing;
        vec![GameEvent::PhaseChanged(Phase::Playing)]
    }

    /// タブが非表示になったとき。プレイ中は一時停止、カウントダウン中はタイトルへ戻す。
    pub fn on_hidden(&mut self, now_ms: f64) -> Vec<GameEvent> {
        match self.phase {
            Phase::Playing => self.pause(now_ms),
            Phase::Countdown => self.back_to_title(),
            _ => Vec::new(),
        }
    }

    /// 向きを切り替える。スロット構成が変わるので的は消し、プレイ中なら一時停止する。
    pub fn set_orientation(&mut self, o: Orientation, now_ms: f64) -> Vec<GameEvent> {
        if o == self.orientation() {
            return Vec::new();
        }
        self.layout = layout_for(o);
        self.targets.clear();
        self.slot_free_at = vec![0.0; self.layout.slot_count()];
        self.spawner.last_target_slot = None;
        self.effects.clear();
        self.aim = None;
        self.pause(now_ms)
    }

    /// 時間を進める。`dt` は秒で、`MAX_DT` に制限する。
    pub fn update(&mut self, now_ms: f64, dt: f64) -> Vec<GameEvent> {
        let dt = dt.clamp(0.0, MAX_DT);
        let mut events = Vec::new();
        match self.phase {
            Phase::Paused => return events,
            Phase::Title => {
                self.tick_world(dt);
                self.run_demo_spawner(dt);
            }
            Phase::Countdown => {
                let before = (self.countdown_t / COUNTDOWN_STEP_SEC) as u8;
                self.countdown_t += dt;
                let after = (self.countdown_t / COUNTDOWN_STEP_SEC) as u8;
                if after > before {
                    if after < COUNT_FROM {
                        events.push(GameEvent::CountdownTick(COUNT_FROM - after));
                    } else {
                        // GO! の表示開始と同時にタイマー開始・的の出現開始。
                        let over = self.countdown_t - COUNTDOWN_STEP_SEC * f64::from(COUNT_FROM);
                        self.phase = Phase::Playing;
                        self.timer.start = Some(now_ms - over * 1000.0);
                        self.play_t = over;
                        events.push(GameEvent::Go);
                        events.push(GameEvent::PhaseChanged(Phase::Playing));
                        self.run_spawner(0.0);
                    }
                }
                self.tick_world(dt);
            }
            Phase::Playing => {
                self.play_t += dt;
                self.tick_world(dt);
                self.run_spawner(dt);
            }
            Phase::Clearing => {
                self.clear_t += dt;
                self.tick_world(dt);
                if self.clear_t >= CLEAR_DELAY_SEC {
                    self.phase = Phase::Result;
                    events.push(GameEvent::PhaseChanged(Phase::Result));
                }
            }
            Phase::Result => self.tick_world(dt),
        }
        events
    }

    /// 的と演出を進め、消えた的を取り除く。
    fn tick_world(&mut self, dt: f64) {
        self.clock += dt;
        for t in &mut self.targets {
            t.update(dt);
        }
        let layout = self.layout;
        let free_at = self.clock + SLOT_COOLDOWN_SEC;
        for t in self.targets.iter().filter(|t| t.is_dead()) {
            self.slot_free_at[layout.slot_index(t.slot)] = free_at;
        }
        self.targets.retain(|t| !t.is_dead());
        self.effects.update(dt);
    }

    /// 撃つ。プレイ中以外は `None`。
    pub fn shoot(&mut self, x: f64, y: f64, touch: bool, now_ms: f64) -> Option<ShotResult> {
        if self.phase != Phase::Playing {
            return None;
        }
        if touch {
            self.effects.tap(x, y);
        }
        let Some(i) = pick_target(&self.targets, self.layout, x, y, touch) else {
            self.miss_count += 1;
            self.effects.hole(x, y);
            return Some(ShotResult::Miss);
        };

        let (cx, cy) = self.targets[i].plate_center();
        let scale = self.targets[i].scale;
        let correct = Some(self.targets[i].glyph) == self.current_glyph();
        self.targets[i].hit(correct);

        if !correct {
            self.penalty_count += 1;
            self.effects
                .shards(&mut self.rng, cx, cy, scale, ParticleColor::Gray);
            self.effects.popup("+2.0", cx, cy - 6.0, PopupKind::Penalty);
            self.effects.shake();
            return Some(ShotResult::Wrong);
        }

        self.progress += 1;
        self.spawner.last_target_seen_at = self.play_t;
        self.effects
            .shards(&mut self.rng, cx, cy, scale, ParticleColor::White);
        self.effects.star((cx, cy), (self.layout.w() / 2.0, 4.0));
        let set = completed_set(self.progress);
        if set.is_some() {
            self.effects.popup(
                "404!",
                self.layout.w() / 2.0,
                self.layout.h() * 0.4,
                PopupKind::Good,
            );
        }
        let cleared = self.progress >= SEQUENCE_LEN;
        if cleared {
            self.timer.end = Some(now_ms);
            self.phase = Phase::Clearing;
            self.clear_t = 0.0;
            for t in &mut self.targets {
                t.force_sink();
            }
            self.effects.confetti(&mut self.rng, self.layout.w());
        }
        Some(ShotResult::Correct { set, cleared })
    }
}

#[cfg(test)]
mod tests {
    use super::super::config::{
        RISE_SEC, TARGET_GAP_MAX, TARGET_MIN_UP_SEC, UP_MAX_SEC, UP_MIN_SEC, VISIBLE_LIMIT,
    };
    use super::super::target::TargetState;
    use super::*;
    use std::collections::HashSet;

    const DT: f64 = 1.0 / 60.0;

    /// カウントダウンを終えてプレイ中にする。戻り値は現在時刻（ms）。
    fn start_playing(g: &mut GameState) -> f64 {
        g.start_countdown();
        let mut now = 0.0;
        while g.phase != Phase::Playing {
            now += DT * 1000.0;
            g.update(now, DT);
        }
        now
    }

    /// 表示中の正解の的の板の中心。
    fn target_point(g: &GameState) -> Option<(f64, f64)> {
        let cur = g.current_glyph()?;
        g.targets
            .iter()
            .find(|t| t.glyph == cur && t.is_hittable())
            .map(|t| t.plate_center())
    }

    fn check_invariants(g: &GameState) {
        let visible: Vec<_> = g.targets.iter().filter(|t| t.is_visible()).collect();
        assert!(visible.len() <= VISIBLE_LIMIT);
        let glyphs: HashSet<_> = visible.iter().map(|t| t.glyph).collect();
        assert_eq!(glyphs.len(), visible.len(), "同じ文字が同時に出ている");
        let slots: HashSet<_> = g.targets.iter().map(|t| t.slot).collect();
        assert_eq!(slots.len(), g.targets.len(), "1 スロットに 2 個出ている");
    }

    #[test]
    fn countdown_then_playing() {
        let mut g = GameState::new(1, Orientation::Landscape);
        let ev = g.start_countdown();
        assert!(ev.contains(&GameEvent::CountdownTick(3)));
        assert_eq!(g.count_label(), Some(CountLabel::Num(3)));
        let mut ticks = Vec::new();
        let mut now = 0.0;
        while g.phase == Phase::Countdown {
            now += DT * 1000.0;
            ticks.extend(g.update(now, DT));
            assert!(g.targets.is_empty() || g.phase == Phase::Playing);
        }
        assert_eq!(
            ticks,
            vec![
                GameEvent::CountdownTick(2),
                GameEvent::CountdownTick(1),
                GameEvent::Go,
                GameEvent::PhaseChanged(Phase::Playing),
            ]
        );
        assert_eq!(g.count_label(), Some(CountLabel::Go));
        // GO と同時に最低数（2 個）が出て、その中に正解がある。
        assert_eq!(g.targets.len(), 2);
        assert!(g.targets.iter().any(|t| t.glyph == Glyph::D4));
        assert!((now - 2100.0).abs() <= DT * 1000.0 + 1e-6);
    }

    #[test]
    fn correct_order_clears_and_wrong_glyph_is_penalized() {
        let mut g = GameState::new(7, Orientation::Landscape);
        let mut now = start_playing(&mut g);
        let mut wrong_done = false;
        let mut frames = 0;
        while g.phase == Phase::Playing {
            frames += 1;
            assert!(frames < 60 * 120, "クリアできない");
            now += DT * 1000.0;
            g.update(now, DT);
            check_invariants(&g);
            // 1 回だけ、わざと違う的を撃つ。
            if !wrong_done {
                let cur = g.current_glyph().unwrap();
                if let Some(t) = g.targets.iter().find(|t| t.glyph != cur && t.is_hittable()) {
                    let (x, y) = t.plate_center();
                    let before = g.progress;
                    assert_eq!(g.shoot(x, y, false, now), Some(ShotResult::Wrong));
                    assert_eq!(g.progress, before);
                    assert_eq!(g.penalty_count, 1);
                    wrong_done = true;
                    continue;
                }
            }
            if let Some((x, y)) = target_point(&g) {
                let before = g.progress;
                match g.shoot(x, y, false, now) {
                    Some(ShotResult::Correct { set, cleared }) => {
                        assert_eq!(g.progress, before + 1);
                        assert_eq!(set, completed_set(g.progress));
                        assert_eq!(cleared, g.progress == SEQUENCE_LEN);
                    }
                    other => panic!("unexpected {other:?}"),
                }
            }
        }
        assert!(wrong_done);
        assert_eq!(g.phase, Phase::Clearing);
        assert_eq!(g.progress, SEQUENCE_LEN);
        let shown = g.display_time(now);
        assert!((shown - g.elapsed(now) - PENALTY_SEC).abs() < 1e-9);
        // タイマーは止まっている。
        assert_eq!(g.display_time(now + 5000.0), shown);

        let mut ev = Vec::new();
        for _ in 0..60 {
            now += DT * 1000.0;
            ev.extend(g.update(now, DT));
        }
        assert_eq!(ev, vec![GameEvent::PhaseChanged(Phase::Result)]);
        assert_eq!(g.phase, Phase::Result);
        assert!(g.shoot(10.0, 10.0, false, now).is_none());
    }

    #[test]
    fn zero_and_oh_are_distinct() {
        let mut g = GameState::new(3, Orientation::Landscape);
        let now = start_playing(&mut g);
        g.progress = 1; // 次は数字の 0
        g.targets.clear();
        let l = g.layout;
        let slot = l.slots().last().unwrap();
        let (x, y) = l.slot_root(slot);
        let mut t = Target::new(99, Glyph::O, slot, (x, y), y, l.row(slot).scale, 1.0);
        t.update(0.2);
        let (cx, cy) = t.plate_center();
        g.targets.push(t);
        assert_eq!(g.shoot(cx, cy, false, now), Some(ShotResult::Wrong));
        assert_eq!(g.progress, 1);
    }

    #[test]
    fn miss_shot_has_no_penalty() {
        let mut g = GameState::new(5, Orientation::Landscape);
        let now = start_playing(&mut g);
        g.targets.clear();
        assert_eq!(g.shoot(1.0, 1.0, true, now), Some(ShotResult::Miss));
        assert_eq!(g.penalty_count, 0);
        assert_eq!(g.display_time(now), g.elapsed(now));
        assert_eq!(g.effects.holes.len(), 1);
    }

    #[test]
    fn spawner_invariants_over_long_runs() {
        for o in [Orientation::Landscape, Orientation::Portrait] {
            for seed in 1..=20u64 {
                let mut g = GameState::new(seed, o);
                let mut now = start_playing(&mut g);
                let mut gap = 0.0f64;
                let mut max_gap = 0.0f64;
                let mut seen: HashSet<u32> = g.targets.iter().map(|t| t.id).collect();
                for frame in 0..10_000 {
                    now += DT * 1000.0;
                    g.update(now, DT);
                    check_invariants(&g);
                    let visible = g.visible_count();
                    if g.play_t > 0.5 {
                        assert!(visible >= 2, "{o:?} seed {seed} frame {frame}: {visible}");
                    }
                    let cur = g.current_glyph().unwrap();
                    let present: Vec<_> = g
                        .targets
                        .iter()
                        .filter(|t| t.is_visible() && t.glyph == cur)
                        .collect();
                    assert!(present.len() <= 1);
                    if present.is_empty() {
                        gap += DT;
                        max_gap = max_gap.max(gap);
                    } else {
                        gap = 0.0;
                    }
                    // 新しく出た的の Up 時間。出現時点で正解だった的は下限が長い。
                    for t in g.targets.iter().filter(|t| seen.insert(t.id)) {
                        assert!(matches!(t.state, TargetState::Rising { .. }));
                        let lo = if t.glyph == cur {
                            TARGET_MIN_UP_SEC
                        } else {
                            UP_MIN_SEC
                        };
                        assert!((lo..UP_MAX_SEC).contains(&t.up_dur), "{}", t.up_dur);
                    }
                    // 時々正解を撃って、文字を入れ替える（クリアはさせない）。
                    if frame % 97 == 0 && g.progress < SEQUENCE_LEN - 1 {
                        if let Some((x, y)) = target_point(&g) {
                            g.shoot(x, y, false, now);
                        }
                    }
                }
                assert!(
                    max_gap <= TARGET_GAP_MAX + RISE_SEC + 2.0 * DT,
                    "{o:?} seed {seed}: gap {max_gap}"
                );
            }
        }
    }

    #[test]
    fn slot_cooldown_is_respected() {
        let mut g = GameState::new(11, Orientation::Portrait);
        let mut now = start_playing(&mut g);
        let mut last_gone: std::collections::HashMap<_, f64> = Default::default();
        let mut alive: HashSet<u32> = HashSet::new();
        let mut slot_of: std::collections::HashMap<u32, _> = Default::default();
        for _ in 0..5_000 {
            now += DT * 1000.0;
            g.update(now, DT);
            let ids: HashSet<u32> = g.targets.iter().map(|t| t.id).collect();
            for gone in alive.difference(&ids) {
                last_gone.insert(slot_of[gone], g.clock);
            }
            for t in &g.targets {
                if !alive.contains(&t.id) {
                    if let Some(at) = last_gone.get(&t.slot) {
                        assert!(g.clock - at >= SLOT_COOLDOWN_SEC - DT - 1e-9);
                    }
                    slot_of.insert(t.id, t.slot);
                }
            }
            alive = ids;
        }
    }

    #[test]
    fn pause_excludes_time() {
        let mut g = GameState::new(2, Orientation::Landscape);
        let start = start_playing(&mut g);
        let t0 = g.elapsed(start);
        g.update(start + 1000.0, DT);
        g.pause(start + 1000.0);
        assert_eq!(g.phase, Phase::Paused);
        // 一時停止中は進まない。
        assert!((g.elapsed(start + 5000.0) - (t0 + 1.0)).abs() < 1e-6);
        let n = g.targets.len();
        g.update(start + 5000.0, DT);
        assert_eq!(g.targets.len(), n);
        g.resume(start + 5000.0);
        assert!((g.elapsed(start + 6000.0) - (t0 + 2.0)).abs() < 1e-6);
    }

    #[test]
    fn hidden_during_countdown_returns_to_title() {
        let mut g = GameState::new(2, Orientation::Landscape);
        g.start_countdown();
        g.update(100.0, DT);
        g.on_hidden(100.0);
        assert_eq!(g.phase, Phase::Title);
    }

    #[test]
    fn rotation_pauses_and_clears_targets() {
        let mut g = GameState::new(4, Orientation::Landscape);
        let mut now = start_playing(&mut g);
        for _ in 0..60 {
            now += DT * 1000.0;
            g.update(now, DT);
        }
        assert!(!g.targets.is_empty());
        g.set_orientation(Orientation::Portrait, now);
        assert_eq!(g.phase, Phase::Paused);
        assert!(g.targets.is_empty());
        assert_eq!(g.slot_free_at.len(), 10);
        g.resume(now);
        now += DT * 1000.0;
        g.update(now, DT);
        assert!(g.visible_count() >= 2);
    }

    #[test]
    fn deterministic_with_same_seed_and_inputs() {
        let run = |seed| {
            let mut g = GameState::new(seed, Orientation::Landscape);
            let mut now = start_playing(&mut g);
            let mut log = Vec::new();
            for frame in 0..3_000 {
                now += DT * 1000.0;
                g.update(now, DT);
                if frame % 7 == 0 {
                    log.push(g.shoot(160.0, 140.0, frame % 2 == 0, now));
                }
                log.push(None);
                if g.phase != Phase::Playing {
                    break;
                }
            }
            (log, g.progress, g.penalty_count, g.next_target_id)
        };
        assert_eq!(run(123), run(123));
    }

    #[test]
    fn title_demo_spawns_targets() {
        let mut g = GameState::new(9, Orientation::Landscape);
        for i in 0..600 {
            g.update(f64::from(i) * 16.0, DT);
            check_invariants(&g);
        }
        assert!(!g.targets.is_empty());
        assert!(g.shoot(160.0, 140.0, false, 0.0).is_none());
    }

    #[test]
    fn notfound_part_shows_more_targets() {
        use super::super::config::{NOTFOUND_START, VISIBLE_MAX};
        for o in [Orientation::Landscape, Orientation::Portrait] {
            for seed in 1..=10u64 {
                let mut g = GameState::new(seed, o);
                let mut now = start_playing(&mut g);
                let mut run = |g: &mut GameState, now: &mut f64| {
                    let mut peak = 0;
                    for _ in 0..3_000 {
                        *now += DT * 1000.0;
                        g.update(*now, DT);
                        peak = peak.max(g.visible_count());
                    }
                    peak
                };
                let before = run(&mut g, &mut now);
                assert!(before <= VISIBLE_MAX, "{o:?} seed {seed}: {before}");
                g.progress = NOTFOUND_START;
                let after = run(&mut g, &mut now);
                assert!(after > VISIBLE_MAX, "{o:?} seed {seed}: {after}");
                assert!(after <= VISIBLE_LIMIT, "{o:?} seed {seed}: {after}");
            }
        }
    }
}
