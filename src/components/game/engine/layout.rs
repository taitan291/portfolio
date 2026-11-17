//! 論理解像度・列（奥行き）・スロットの定義（設計書 6 章）。

use super::config::{JITTER_X, MAX_OVERLAP_RATIO, PLATE_D};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Orientation {
    Landscape,
    Portrait,
}

impl Orientation {
    /// 表示領域の幅・高さから向きを決める。
    pub fn from_size(w: f64, h: f64) -> Self {
        if w >= h {
            Self::Landscape
        } else {
            Self::Portrait
        }
    }
}

/// 1 列分の棚。
#[derive(Debug)]
pub struct Row {
    /// 棚板の上端の y 座標。的の根元はここに来る。
    pub y: f64,
    /// 的の縮尺。
    pub scale: f64,
    /// 明度（1.0 = そのまま）。
    pub brightness: f64,
    /// 的の文字のドット倍率（整数）。
    pub text_scale: u32,
    /// 棚板の厚み。
    pub shelf_h: f64,
    /// スロットの x 座標（的の根元の中央）。
    pub slot_xs: &'static [f64],
}

#[derive(Debug)]
pub struct Layout {
    pub orientation: Orientation,
    pub width: u32,
    pub height: u32,
    /// 奥から手前の順。
    pub rows: &'static [Row],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct SlotId {
    pub row: u8,
    pub col: u8,
}

static LANDSCAPE: Layout = Layout {
    orientation: Orientation::Landscape,
    width: 320,
    height: 180,
    rows: &[
        Row {
            y: 78.0,
            scale: 0.60,
            brightness: 0.80,
            text_scale: 2,
            shelf_h: 8.0,
            slot_xs: &[52.0, 108.0, 164.0, 220.0, 276.0],
        },
        Row {
            y: 118.0,
            scale: 0.80,
            brightness: 0.90,
            text_scale: 2,
            shelf_h: 8.0,
            slot_xs: &[64.0, 128.0, 192.0, 256.0],
        },
        Row {
            y: 166.0,
            scale: 1.00,
            brightness: 1.00,
            text_scale: 3,
            shelf_h: 14.0,
            slot_xs: &[80.0, 160.0, 240.0],
        },
    ],
};

static PORTRAIT: Layout = Layout {
    orientation: Orientation::Portrait,
    width: 180,
    height: 320,
    rows: &[
        Row {
            y: 112.0,
            scale: 0.60,
            brightness: 0.78,
            text_scale: 2,
            shelf_h: 8.0,
            slot_xs: &[36.0, 90.0, 144.0],
        },
        Row {
            y: 168.0,
            scale: 0.73,
            brightness: 0.85,
            text_scale: 2,
            shelf_h: 8.0,
            slot_xs: &[36.0, 90.0, 144.0],
        },
        Row {
            y: 230.0,
            scale: 0.86,
            brightness: 0.92,
            text_scale: 2,
            shelf_h: 8.0,
            slot_xs: &[45.0, 135.0],
        },
        Row {
            y: 300.0,
            scale: 1.00,
            brightness: 1.00,
            text_scale: 3,
            shelf_h: 20.0,
            slot_xs: &[45.0, 135.0],
        },
    ],
};

pub fn layout_for(o: Orientation) -> &'static Layout {
    match o {
        Orientation::Landscape => &LANDSCAPE,
        Orientation::Portrait => &PORTRAIT,
    }
}

impl Layout {
    pub fn w(&self) -> f64 {
        f64::from(self.width)
    }

    pub fn h(&self) -> f64 {
        f64::from(self.height)
    }

    pub fn slot_count(&self) -> usize {
        self.rows.iter().map(|r| r.slot_xs.len()).sum()
    }

    /// 全スロットを奥の列から順に列挙する。
    pub fn slots(&self) -> impl Iterator<Item = SlotId> + '_ {
        self.rows.iter().enumerate().flat_map(|(r, row)| {
            (0..row.slot_xs.len()).map(move |c| SlotId {
                row: r as u8,
                col: c as u8,
            })
        })
    }

    /// スロットの通し番号（`slots()` の順）。
    pub fn slot_index(&self, slot: SlotId) -> usize {
        let before: usize = self.rows[..usize::from(slot.row)]
            .iter()
            .map(|r| r.slot_xs.len())
            .sum();
        before + usize::from(slot.col)
    }

    pub fn row(&self, slot: SlotId) -> &Row {
        &self.rows[usize::from(slot.row)]
    }

    /// スロットの根元の座標（ジッターなし）。
    pub fn slot_root(&self, slot: SlotId) -> (f64, f64) {
        let row = self.row(slot);
        (row.slot_xs[usize::from(slot.col)], row.y)
    }

    /// その列で使える X ジッターの最大値。隣の的と幅の 15% 以上重ならず、画面からはみ出さない。
    pub fn max_jitter_x(&self, row: &Row) -> f64 {
        let width = PLATE_D * row.scale;
        let mut limit = JITTER_X * row.scale;
        for pair in row.slot_xs.windows(2) {
            // 隣同士が逆向きに最大までずれても、重なりが幅の 15% 以内に収まる量。
            let room = (pair[1] - pair[0] - width * (1.0 - MAX_OVERLAP_RATIO)) / 2.0;
            limit = limit.min(room.max(0.0));
        }
        if let (Some(first), Some(last)) = (row.slot_xs.first(), row.slot_xs.last()) {
            limit = limit
                .min((first - width / 2.0).max(0.0))
                .min((self.w() - last - width / 2.0).max(0.0));
        }
        limit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::game::engine::config::TARGET_H;

    #[test]
    fn slot_counts() {
        assert_eq!(layout_for(Orientation::Landscape).slot_count(), 12);
        assert_eq!(layout_for(Orientation::Portrait).slot_count(), 10);
    }

    #[test]
    fn slot_index_matches_iteration_order() {
        for o in [Orientation::Landscape, Orientation::Portrait] {
            let l = layout_for(o);
            for (i, s) in l.slots().enumerate() {
                assert_eq!(l.slot_index(s), i);
            }
        }
    }

    #[test]
    fn rows_do_not_overlap_and_fit() {
        for o in [Orientation::Landscape, Orientation::Portrait] {
            let l = layout_for(o);
            for pair in l.rows.windows(2) {
                let (back, front) = (&pair[0], &pair[1]);
                assert!(back.scale < front.scale);
                // 手前の的が完全に出ても、奥の棚板にかからない。
                assert!(front.y - TARGET_H * front.scale >= back.y + back.shelf_h);
            }
            let last = l.rows.last().unwrap();
            assert!(last.y + last.shelf_h <= l.h());
        }
    }

    #[test]
    fn orientation_from_size() {
        assert_eq!(Orientation::from_size(800.0, 600.0), Orientation::Landscape);
        assert_eq!(Orientation::from_size(500.0, 500.0), Orientation::Landscape);
        assert_eq!(Orientation::from_size(390.0, 844.0), Orientation::Portrait);
    }
}
