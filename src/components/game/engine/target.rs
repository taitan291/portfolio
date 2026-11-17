//! 的（ターゲット）とそのライフサイクル（設計書 7.1）。

use super::config::{HIT_ANIM_SEC, PLATE_D, RISE_SEC, SINK_SEC, TARGET_H};
use super::layout::SlotId;
use super::sequence::Glyph;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TargetState {
    /// 棚板の裏からせり上がり中。`t`: 0.0..RISE_SEC
    Rising { t: f64 },
    /// 出ている。`dur` 経過で引っ込み始める。
    Up { t: f64, dur: f64 },
    /// 引っ込み中。`t`: 0.0..SINK_SEC
    Sinking { t: f64 },
    /// 撃たれて倒れている途中。
    Hit { t: f64, correct: bool },
    /// 消滅。次の更新で取り除かれる。
    Gone,
}

#[derive(Clone, Debug)]
pub struct Target {
    pub id: u32,
    pub glyph: Glyph,
    pub slot: SlotId,
    /// ジッター込みの根元の座標。ジッターは上方向のみなので `root.1 <= shelf_y`。
    pub root: (f64, f64),
    /// 棚板の上端の y 座標。これより下は隠れる。
    pub shelf_y: f64,
    pub scale: f64,
    /// `Up` の時間。
    pub up_dur: f64,
    pub state: TargetState,
}

fn ease_out(t: f64) -> f64 {
    1.0 - (1.0 - t) * (1.0 - t)
}

fn ease_in(t: f64) -> f64 {
    t * t
}

impl Target {
    pub fn new(
        id: u32,
        glyph: Glyph,
        slot: SlotId,
        root: (f64, f64),
        shelf_y: f64,
        scale: f64,
        up_dur: f64,
    ) -> Self {
        Self {
            id,
            glyph,
            slot,
            root,
            shelf_y,
            scale,
            up_dur,
            state: TargetState::Rising { t: 0.0 },
        }
    }

    /// 0.0（完全に隠れている）..1.0（完全に出ている）。
    pub fn raise_ratio(&self) -> f64 {
        match self.state {
            TargetState::Rising { t } => ease_out((t / RISE_SEC).clamp(0.0, 1.0)),
            TargetState::Up { .. } | TargetState::Hit { .. } => 1.0,
            TargetState::Sinking { t } => 1.0 - ease_in((t / SINK_SEC).clamp(0.0, 1.0)),
            TargetState::Gone => 0.0,
        }
    }

    /// 当たり判定の対象か。せり上がりは 50% から、引っ込みは 50% まで。
    pub fn is_hittable(&self) -> bool {
        match self.state {
            TargetState::Rising { t } => t >= RISE_SEC * 0.5,
            TargetState::Up { .. } => true,
            TargetState::Sinking { t } => t <= SINK_SEC * 0.5,
            TargetState::Hit { .. } | TargetState::Gone => false,
        }
    }

    /// 表示中か（同時表示数に数えるか）。撃たれた的は数えない。
    pub fn is_visible(&self) -> bool {
        matches!(
            self.state,
            TargetState::Rising { .. } | TargetState::Up { .. } | TargetState::Sinking { .. }
        )
    }

    pub fn is_dead(&self) -> bool {
        self.state == TargetState::Gone
    }

    /// 板の半径。
    pub fn plate_radius(&self) -> f64 {
        PLATE_D * self.scale / 2.0
    }

    /// 現在のせり上がり量を反映した板の中心。
    pub fn plate_center(&self) -> (f64, f64) {
        let h = TARGET_H * self.scale;
        // 完全に隠れたとき、板の上端がちょうど棚板の上端に来る。
        let drop = (1.0 - self.raise_ratio()) * (h + self.shelf_y - self.root.1);
        (self.root.0, self.root.1 - h + self.plate_radius() + drop)
    }

    /// 引っ込みを始める（クリア時など）。撃たれた的・引っ込み中の的はそのまま。
    pub fn force_sink(&mut self) {
        match self.state {
            TargetState::Rising { t } => {
                // 現在の高さから下がり始めるよう、引っ込みの経過時間を合わせる。
                let r = ease_out((t / RISE_SEC).clamp(0.0, 1.0));
                let t = ((1.0 - r).sqrt() * SINK_SEC).clamp(0.0, SINK_SEC);
                self.state = TargetState::Sinking { t };
            }
            TargetState::Up { .. } => self.state = TargetState::Sinking { t: 0.0 },
            _ => {}
        }
    }

    pub fn hit(&mut self, correct: bool) {
        self.state = TargetState::Hit { t: 0.0, correct };
    }

    /// 時間を進める。余った時間は次の状態へ持ち越す。
    pub fn update(&mut self, dt: f64) {
        let mut rest = dt;
        // 1 フレームで複数の状態をまたぐこともある。
        loop {
            match &mut self.state {
                TargetState::Rising { t } => {
                    *t += rest;
                    if *t < RISE_SEC {
                        return;
                    }
                    rest = *t - RISE_SEC;
                    self.state = TargetState::Up {
                        t: 0.0,
                        dur: self.up_dur,
                    };
                }
                TargetState::Up { t, dur } => {
                    *t += rest;
                    if *t < *dur {
                        return;
                    }
                    rest = *t - *dur;
                    self.state = TargetState::Sinking { t: 0.0 };
                }
                TargetState::Sinking { t } => {
                    *t += rest;
                    if *t >= SINK_SEC {
                        self.state = TargetState::Gone;
                    }
                    return;
                }
                TargetState::Hit { t, .. } => {
                    *t += rest;
                    if *t >= HIT_ANIM_SEC {
                        self.state = TargetState::Gone;
                    }
                    return;
                }
                TargetState::Gone => return,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn target() -> Target {
        Target::new(
            1,
            Glyph::D4,
            SlotId { row: 0, col: 0 },
            (100.0, 100.0),
            100.0,
            1.0,
            0.5,
        )
    }

    #[test]
    fn lifecycle() {
        let mut t = target();
        assert!(!t.is_hittable());
        assert_eq!(t.raise_ratio(), 0.0);
        t.update(0.06);
        assert!(t.is_hittable());
        t.update(0.05);
        assert!(matches!(t.state, TargetState::Up { .. }));
        assert_eq!(t.raise_ratio(), 1.0);
        t.update(0.48);
        assert!(matches!(t.state, TargetState::Up { .. }));
        t.update(0.02);
        assert!(matches!(t.state, TargetState::Sinking { .. }));
        assert!(t.is_hittable());
        t.update(0.05);
        assert!(!t.is_hittable());
        assert!(t.is_visible());
        t.update(0.05);
        assert!(t.is_dead());
    }

    #[test]
    fn hit_then_gone() {
        let mut t = target();
        t.update(0.2);
        t.hit(true);
        assert!(!t.is_visible());
        assert!(!t.is_hittable());
        t.update(HIT_ANIM_SEC);
        assert!(t.is_dead());
    }

    #[test]
    fn plate_center_moves_with_raise() {
        let mut t = target();
        let (_, hidden_y) = t.plate_center();
        t.update(1.0 / 60.0 * 10.0);
        let (_, up_y) = t.plate_center();
        assert!(up_y < hidden_y);
        assert_eq!(up_y, 100.0 - TARGET_H + PLATE_D / 2.0);
    }
}
