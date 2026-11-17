//! 当たり判定（設計書 8.3）。

use super::config::{HIT_RADIUS, TOUCH_HIT_BONUS};
use super::layout::Layout;
use super::target::Target;

/// `(x, y)` に当たる的の添字を返す。
///
/// 手前の列から奥へ、同じ列では後から出た的を優先し、最初に当たった 1 個だけを返す（貫通しない）。
/// 棚板より下（的が隠れている部分）は判定しない。
pub fn pick_target(
    targets: &[Target],
    layout: &Layout,
    x: f64,
    y: f64,
    touch: bool,
) -> Option<usize> {
    let bonus = if touch { TOUCH_HIT_BONUS } else { 0.0 };
    targets
        .iter()
        .enumerate()
        .filter(|(_, t)| t.is_hittable())
        .filter(|(_, t)| y < layout.row(t.slot).y)
        .filter(|(_, t)| {
            let (cx, cy) = t.plate_center();
            let r = HIT_RADIUS * t.scale + bonus;
            (x - cx).powi(2) + (y - cy).powi(2) <= r * r
        })
        .max_by_key(|(_, t)| (t.slot.row, t.id))
        .map(|(i, _)| i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::game::engine::layout::{layout_for, Orientation, SlotId};
    use crate::components::game::engine::sequence::Glyph;

    fn up_target(id: u32, row: u8, root: (f64, f64), scale: f64) -> Target {
        let mut t = Target::new(
            id,
            Glyph::D4,
            SlotId { row, col: 0 },
            root,
            root.1,
            scale,
            1.0,
        );
        t.update(0.2);
        t
    }

    #[test]
    fn front_row_wins_and_only_one_hit() {
        let l = layout_for(Orientation::Landscape);
        // 中の列と手前の列の的を、わざと重なる位置に置く。
        let back = up_target(1, 1, (100.0, 118.0), 0.8);
        let front = up_target(2, 2, (100.0, 120.0), 1.0);
        let (cx, cy) = (100.0, 96.0);
        let ts = vec![front, back];
        assert_eq!(pick_target(&ts, l, cx, cy, false), Some(0));
        let ts: Vec<_> = ts.into_iter().rev().collect();
        assert_eq!(pick_target(&ts, l, cx, cy, false), Some(1));
    }

    #[test]
    fn later_target_wins_in_same_row() {
        let l = layout_for(Orientation::Landscape);
        let a = up_target(1, 1, (128.0, 118.0), 0.8);
        let b = up_target(5, 1, (130.0, 118.0), 0.8);
        let (cx, cy) = a.plate_center();
        assert_eq!(pick_target(&[b, a], l, cx, cy, false), Some(0));
    }

    #[test]
    fn below_shelf_and_radius() {
        let l = layout_for(Orientation::Landscape);
        // 手前の列（棚板 y = 166）。
        let t = up_target(1, 2, (160.0, 166.0), 1.0);
        let (cx, cy) = t.plate_center();
        let ts = [t];
        assert_eq!(pick_target(&ts, l, cx + HIT_RADIUS, cy, false), Some(0));
        assert_eq!(pick_target(&ts, l, cx + HIT_RADIUS + 1.0, cy, false), None);
        assert_eq!(
            pick_target(&ts, l, cx + HIT_RADIUS + 1.0, cy, true),
            Some(0)
        );
        assert_eq!(pick_target(&ts, l, cx, 167.0, true), None);
    }
}
