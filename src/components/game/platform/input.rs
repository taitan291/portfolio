//! ポインタ入力の座標変換と連射制限（設計書 8 章）。

use web_sys::HtmlCanvasElement;

use crate::components::game::engine::config::FIRE_COOLDOWN_MS;

/// クライアント座標を論理座標に変換する。表示サイズの変化に追従するよう毎回計算する。
pub fn to_logical(
    canvas: &HtmlCanvasElement,
    client_x: f64,
    client_y: f64,
    logical_w: f64,
    logical_h: f64,
) -> Option<(f64, f64)> {
    let rect = canvas.get_bounding_client_rect();
    if rect.width() <= 0.0 || rect.height() <= 0.0 {
        return None;
    }
    Some((
        (client_x - rect.left()) / rect.width() * logical_w,
        (client_y - rect.top()) / rect.height() * logical_h,
    ))
}

/// 同一ポインタで `FIRE_COOLDOWN_MS` 以内の連続ショットを無視する。
#[derive(Default)]
pub struct FireLimiter {
    last: Vec<(i32, f64)>,
}

impl FireLimiter {
    pub fn allow(&mut self, pointer_id: i32, now_ms: f64) -> bool {
        // 古い記録は捨てる（マルチタッチでポインタ ID が増え続けるため）。
        self.last.retain(|(_, t)| now_ms - t < FIRE_COOLDOWN_MS);
        if self.last.iter().any(|(id, _)| *id == pointer_id) {
            return false;
        }
        self.last.push((pointer_id, now_ms));
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limiter_per_pointer() {
        let mut l = FireLimiter::default();
        assert!(l.allow(1, 0.0));
        assert!(!l.allow(1, 30.0));
        assert!(l.allow(2, 30.0));
        assert!(l.allow(1, 61.0));
    }
}
