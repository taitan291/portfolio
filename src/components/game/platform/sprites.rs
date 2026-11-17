//! スプライト（的の板・文字・背景）の生成とキャッシュ（設計書 10.4・17 章）。
//!
//! すべてコードで描き、非表示の `<canvas>` に焼き込んで `drawImage` でスタンプする。
//! 将来 PNG に差し替える場合も、このキャッシュの中身を入れ替えるだけで済む。

use super::font::{self, GLYPH_H, GLYPH_W};
use super::palette as pal;
use crate::components::game::engine::config::PLATE_D;
use crate::components::game::engine::layout::{Layout, Orientation};
use crate::components::game::engine::rng::Rng;
use crate::components::game::engine::sequence::GlyphKind;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d as Ctx, Document, HtmlCanvasElement};

/// 屋台の柱の幅。棚板もこの内側に描く。
pub const POST_W: f64 = 8.0;

/// 明度を百分率に丸めたもの（キャッシュのキー）。
fn bright_key(b: f64) -> u8 {
    (b * 100.0).round().clamp(0.0, 100.0) as u8
}

pub struct Sprites {
    document: Document,
    plates: HashMap<(GlyphKind, u32, u8), HtmlCanvasElement>,
    glyphs: HashMap<(char, u32, u8), HtmlCanvasElement>,
    background: Option<(Orientation, HtmlCanvasElement)>,
}

impl Sprites {
    pub fn new(document: Document) -> Self {
        Self {
            document,
            plates: HashMap::new(),
            glyphs: HashMap::new(),
            background: None,
        }
    }

    fn canvas(&self, w: u32, h: u32) -> Option<(HtmlCanvasElement, Ctx)> {
        let c: HtmlCanvasElement = self
            .document
            .create_element("canvas")
            .ok()?
            .dyn_into()
            .ok()?;
        c.set_width(w.max(1));
        c.set_height(h.max(1));
        let ctx: Ctx = c.get_context("2d").ok()??.dyn_into().ok()?;
        ctx.set_image_smoothing_enabled(false);
        Some((c, ctx))
    }

    /// 的の板（3 重の同心円）。直径は `PLATE_D * scale` を丸めたもの。
    pub fn plate(
        &mut self,
        kind: GlyphKind,
        scale: f64,
        brightness: f64,
    ) -> Option<&HtmlCanvasElement> {
        let d = (PLATE_D * scale).round().max(8.0) as u32;
        let key = (kind, d, bright_key(brightness));
        if !self.plates.contains_key(&key) {
            let (c, ctx) = self.canvas(d, d)?;
            draw_plate(&ctx, d, kind);
            darken(&ctx, d, d, brightness);
            self.plates.insert(key, c);
        }
        self.plates.get(&key)
    }

    /// 縁取り付きの白い文字。サイズは `(5 * scale + 2) × (7 * scale + 2)`。
    pub fn glyph(&mut self, ch: char, scale: u32, brightness: f64) -> Option<&HtmlCanvasElement> {
        let key = (ch, scale, bright_key(brightness));
        if !self.glyphs.contains_key(&key) {
            let (c, ctx) = self.canvas(GLYPH_W * scale + 2, GLYPH_H * scale + 2)?;
            let s = f64::from(scale);
            ctx.set_fill_style_str(pal::BLACK);
            for dy in -1..=1 {
                for dx in -1..=1 {
                    font::fill_char(&ctx, ch, 1.0 + f64::from(dx), 1.0 + f64::from(dy), s);
                }
            }
            ctx.set_fill_style_str(pal::WHITE);
            font::fill_char(&ctx, ch, 1.0, 1.0, s);
            darken(&ctx, GLYPH_W * scale + 2, GLYPH_H * scale + 2, brightness);
            self.glyphs.insert(key, c);
        }
        self.glyphs.get(&key)
    }

    /// 背景（屋台の幕・壁・柱・ちょうちん）。向きごとに 1 枚作る。
    pub fn background(&mut self, layout: &Layout) -> Option<&HtmlCanvasElement> {
        let stale = self
            .background
            .as_ref()
            .is_none_or(|(o, _)| *o != layout.orientation);
        if stale {
            let (c, ctx) = self.canvas(layout.width, layout.height)?;
            draw_background(&ctx, layout);
            self.background = Some((layout.orientation, c));
        }
        self.background.as_ref().map(|(_, c)| c)
    }
}

/// 描いた内容だけを暗くする。
fn darken(ctx: &Ctx, w: u32, h: u32, brightness: f64) {
    if brightness >= 0.999 {
        return;
    }
    let _ = ctx.set_global_composite_operation("source-atop");
    ctx.set_fill_style_str(&format!(
        "rgba(11, 13, 26, {:.3})",
        (1.0 - brightness).clamp(0.0, 1.0)
    ));
    ctx.fill_rect(0.0, 0.0, f64::from(w), f64::from(h));
    let _ = ctx.set_global_composite_operation("source-over");
}

fn plate_colors(kind: GlyphKind) -> (&'static str, &'static str) {
    match kind {
        GlyphKind::Digit => (pal::RED, pal::CREAM),
        GlyphKind::SeqLetter => (pal::BLUE, pal::CREAM),
        GlyphKind::Decoy => (pal::SAGE, pal::CREAM),
    }
}

/// アンチエイリアスのない、ドット単位の円を描く。
fn draw_plate(ctx: &Ctx, d: u32, kind: GlyphKind) {
    let (main, ring) = plate_colors(kind);
    let r = f64::from(d) / 2.0;
    let mut current = "";
    for py in 0..d {
        for px in 0..d {
            let dist = (f64::from(px) + 0.5 - r).hypot(f64::from(py) + 0.5 - r);
            let color = if dist > r {
                continue;
            } else if dist > r - 1.0 {
                pal::BLACK
            } else if dist > r * 0.70 {
                main
            } else if dist > r * 0.45 {
                ring
            } else {
                main
            };
            if color != current {
                ctx.set_fill_style_str(color);
                current = color;
            }
            ctx.fill_rect(f64::from(px), f64::from(py), 1.0, 1.0);
        }
    }
    // 左上にハイライトを 1 点ずつ入れて、板の丸みを出す。
    ctx.set_fill_style_str("rgba(255, 255, 255, 0.35)");
    let hl = (r * 0.45).floor();
    ctx.fill_rect(hl, hl + 1.0, 1.0, 2.0);
    ctx.fill_rect(hl + 1.0, hl, 2.0, 1.0);
}

/// 幕の高さ。
fn curtain_h(layout: &Layout) -> f64 {
    match layout.orientation {
        Orientation::Landscape => 14.0,
        Orientation::Portrait => 18.0,
    }
}

fn draw_background(ctx: &Ctx, layout: &Layout) {
    let (w, h) = (layout.w(), layout.h());

    // 壁: 上が暗い濃紺、下が紫。境目はチェッカーでディザリングする。
    let bands = [pal::BLACK, pal::NAVY, pal::INDIGO, pal::PURPLE];
    let band_h = (h / bands.len() as f64).ceil();
    for (i, color) in bands.iter().enumerate() {
        ctx.set_fill_style_str(color);
        ctx.fill_rect(0.0, i as f64 * band_h, w, band_h);
        if let Some(next) = bands.get(i + 1) {
            ctx.set_fill_style_str(next);
            let y0 = (i as f64 + 1.0) * band_h - 2.0;
            for dy in 0..2 {
                let mut x = f64::from(dy % 2);
                while x < w {
                    ctx.fill_rect(x, y0 + f64::from(dy), 1.0, 1.0);
                    x += 2.0;
                }
            }
        }
    }
    // 壁板の継ぎ目。
    ctx.set_fill_style_str("rgba(0, 0, 0, 0.18)");
    let mut x = POST_W + 15.0;
    while x < w - POST_W {
        ctx.fill_rect(x, 0.0, 1.0, h);
        x += 16.0;
    }
    // 星のようなちらつき（固定 seed）。
    let mut rng = Rng::new(404);
    let top = curtain_h(layout) + 8.0;
    for _ in 0..(w * h / 900.0) as usize {
        let sx = rng.range(POST_W, w - POST_W).floor();
        let sy = rng.range(top, h * 0.35).floor();
        ctx.set_fill_style_str(if rng.chance(0.3) {
            pal::YELLOW
        } else {
            pal::GRAY
        });
        ctx.fill_rect(sx, sy, 1.0, 1.0);
    }

    draw_lanterns(ctx, layout);

    // 柱。
    for x in [0.0, w - POST_W] {
        ctx.set_fill_style_str(pal::BROWN);
        ctx.fill_rect(x, 0.0, POST_W, h);
        ctx.set_fill_style_str(pal::WOOD);
        ctx.fill_rect(x + 1.0, 0.0, 2.0, h);
        ctx.set_fill_style_str(pal::BLACK);
        ctx.fill_rect(x + POST_W - 1.0, 0.0, 1.0, h);
    }

    draw_curtain(ctx, w, curtain_h(layout));
}

/// 上部の紅白の幕。下端は半円の波形にする。
fn draw_curtain(ctx: &Ctx, w: f64, ch: f64) {
    const STRIPE: f64 = 10.0;
    let mut i = 0;
    let mut x = 0.0;
    while x < w {
        let color = if i % 2 == 0 { pal::RED } else { pal::CREAM };
        ctx.set_fill_style_str(color);
        ctx.fill_rect(x, 0.0, STRIPE, ch);
        // 半円のすそ。
        let r = STRIPE / 2.0;
        for dy in 0..(r as i32) {
            let half = (r * r - (f64::from(dy) + 0.5).powi(2)).max(0.0).sqrt();
            ctx.fill_rect(
                (x + r - half).round(),
                ch + f64::from(dy),
                (half * 2.0).round(),
                1.0,
            );
        }
        x += STRIPE;
        i += 1;
    }
    // 影。
    ctx.set_fill_style_str("rgba(0, 0, 0, 0.35)");
    ctx.fill_rect(0.0, 0.0, w, 2.0);
}

/// 左右のちょうちん。
fn draw_lanterns(ctx: &Ctx, layout: &Layout) {
    let (w, ch) = (layout.w(), curtain_h(layout));
    let xs = [POST_W + 12.0, w - POST_W - 12.0];
    for cx in xs {
        let top = ch + 6.0;
        // 光のにじみ（市松模様のディザで円形に）。
        ctx.set_fill_style_str("rgba(245, 212, 94, 0.16)");
        for dy in -12i32..=12 {
            for dx in -12i32..=12 {
                if dx * dx + dy * dy <= 144 && (dx + dy).rem_euclid(2) == 0 {
                    ctx.fill_rect(cx + f64::from(dx), top + 5.0 + f64::from(dy), 1.0, 1.0);
                }
            }
        }
        ctx.set_fill_style_str(pal::DARK_GRAY);
        ctx.fill_rect(cx, ch, 1.0, 6.0);
        ctx.set_fill_style_str(pal::BLACK);
        ctx.fill_rect(cx - 2.0, top, 5.0, 1.0);
        ctx.fill_rect(cx - 2.0, top + 10.0, 5.0, 1.0);
        ctx.set_fill_style_str(pal::RED);
        ctx.fill_rect(cx - 3.0, top + 1.0, 7.0, 9.0);
        ctx.fill_rect(cx - 4.0, top + 3.0, 9.0, 5.0);
        ctx.set_fill_style_str(pal::ORANGE);
        ctx.fill_rect(cx - 1.0, top + 2.0, 3.0, 7.0);
        ctx.set_fill_style_str(pal::YELLOW);
        ctx.fill_rect(cx, top + 4.0, 1.0, 3.0);
    }
}
