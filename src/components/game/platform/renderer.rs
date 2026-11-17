//! Canvas 2D の描画（設計書 10 章）。論理解像度で描き、CSS で拡大する。

use super::font;
use super::palette as pal;
use super::sprites::{Sprites, POST_W};
use crate::components::game::engine::config::HIT_ANIM_SEC;
use crate::components::game::engine::effects::{ParticleColor, PopupKind, HOLE_SEC, TAP_MARK_SEC};
use crate::components::game::engine::layout::{Layout, Row};
use crate::components::game::engine::state::GameState;
use crate::components::game::engine::target::{Target, TargetState};
use std::f64::consts::TAU;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d as Ctx, HtmlCanvasElement};

pub struct Renderer {
    canvas: HtmlCanvasElement,
    ctx: Ctx,
    sprites: Sprites,
    layout: &'static Layout,
}

impl Renderer {
    pub fn new(canvas: HtmlCanvasElement, layout: &'static Layout) -> Option<Self> {
        let document = web_sys::window()?.document()?;
        let ctx: Ctx = canvas.get_context("2d").ok()??.dyn_into().ok()?;
        let mut r = Self {
            canvas,
            ctx,
            sprites: Sprites::new(document),
            layout,
        };
        r.apply_size();
        Some(r)
    }

    pub fn canvas(&self) -> &HtmlCanvasElement {
        &self.canvas
    }

    fn apply_size(&mut self) {
        self.canvas.set_width(self.layout.width);
        self.canvas.set_height(self.layout.height);
        // サイズを変えるとコンテキストの設定が初期化されるので、毎回設定し直す。
        self.ctx.set_image_smoothing_enabled(false);
    }

    pub fn set_layout(&mut self, layout: &'static Layout) {
        if !std::ptr::eq(self.layout, layout) {
            self.layout = layout;
            self.apply_size();
        }
    }

    pub fn draw(&mut self, state: &GameState, reduced_motion: bool) {
        let layout = self.layout;
        let (w, h) = (layout.w(), layout.h());
        let ctx = &self.ctx;

        ctx.set_fill_style_str(pal::BLACK);
        ctx.fill_rect(0.0, 0.0, w, h);

        ctx.save();
        let (sx, sy) = if reduced_motion {
            (0.0, 0.0)
        } else {
            state.effects.shake_offset(state.clock)
        };
        let _ = ctx.translate(sx, sy);

        if let Some(bg) = self.sprites.background(layout) {
            let _ = ctx.draw_image_with_html_canvas_element(bg, 0.0, 0.0);
        }

        // 奥の列から順に「的 → 棚板」を描く。的は棚板の上端でクリップしてせり上がりを表現する。
        for (r, row) in layout.rows.iter().enumerate() {
            ctx.save();
            ctx.begin_path();
            ctx.rect(-4.0, -4.0, w + 8.0, row.y + 4.0);
            ctx.clip();
            for t in state
                .targets
                .iter()
                .filter(|t| usize::from(t.slot.row) == r)
            {
                draw_target(ctx, &mut self.sprites, t, row, state.clock);
            }
            ctx.restore();
            draw_shelf(ctx, row, w, r + 1 == layout.rows.len());
        }

        draw_effects(ctx, state);
        ctx.restore();

        // 照準は揺らさない。
        for m in &state.effects.taps {
            let k = 1.0 - m.t / TAP_MARK_SEC;
            draw_crosshair(ctx, m.x, m.y, (4.0 * k).round().max(1.0));
        }
        if let Some((x, y)) = state.aim {
            draw_crosshair(ctx, x, y, 4.0);
        }
    }
}

fn draw_target(ctx: &Ctx, sprites: &mut Sprites, t: &Target, row: &Row, clock: f64) {
    let (mut cx, cy) = t.plate_center();
    if matches!(t.state, TargetState::Up { .. }) {
        // わずかに揺らす（±1px、周期 0.5 秒）。
        cx += ((clock * TAU / 0.5) + f64::from(t.id)).sin().round();
    }
    let r = t.plate_radius();

    // 倒れる演出: 板を下端基準で縦に縮める。
    let squash = match t.state {
        TargetState::Hit { t, .. } => (1.0 - t / HIT_ANIM_SEC).clamp(0.0, 1.0),
        _ => 1.0,
    };

    // 支柱。
    let stick_w = (3.0 * t.scale).round().max(2.0);
    let stick_x = (cx - stick_w / 2.0).floor();
    let stick_top = (cy + r - 1.0).floor();
    let stick_h = (t.shelf_y - stick_top).max(0.0);
    ctx.set_fill_style_str(pal::BROWN);
    ctx.fill_rect(stick_x, stick_top, stick_w, stick_h);
    ctx.set_fill_style_str(pal::WOOD);
    ctx.fill_rect(stick_x, stick_top, 1.0, stick_h);

    let kind = t.glyph.kind();
    let bottom = cy + r;
    let Some(plate) = sprites.plate(kind, t.scale, row.brightness) else {
        return;
    };
    let d = f64::from(plate.width());
    let dh = (d * squash).round().max(1.0);
    let px = (cx - d / 2.0).floor();
    let py = (bottom - dh).floor();
    let _ = ctx.draw_image_with_html_canvas_element_and_dw_and_dh(plate, px, py, d, dh);

    let Some(glyph) = sprites.glyph(t.glyph.as_char(), row.text_scale, row.brightness) else {
        return;
    };
    let (gw, gh) = (f64::from(glyph.width()), f64::from(glyph.height()));
    let gdh = (gh * squash).round();
    if gdh < 1.0 {
        return;
    }
    let plate_mid = bottom - dh / 2.0;
    let _ = ctx.draw_image_with_html_canvas_element_and_dw_and_dh(
        glyph,
        (cx - gw / 2.0).floor(),
        (plate_mid - gdh / 2.0).floor(),
        gw,
        gdh,
    );
}

/// 棚板。いちばん手前はカウンター（布を掛けた台）。
fn draw_shelf(ctx: &Ctx, row: &Row, w: f64, counter: bool) {
    let (x0, x1) = (POST_W, w - POST_W);
    let y = row.y;
    ctx.set_fill_style_str(pal::WOOD);
    ctx.fill_rect(x0, y, x1 - x0, 2.0);
    ctx.set_fill_style_str(pal::BROWN);
    ctx.fill_rect(x0, y + 2.0, x1 - x0, row.shelf_h - 2.0);
    if counter {
        // 赤い布と白いライン。
        let cloth_top = y + 4.0;
        ctx.set_fill_style_str(pal::RED);
        ctx.fill_rect(0.0, cloth_top, w, row.shelf_h);
        ctx.set_fill_style_str(pal::CREAM);
        ctx.fill_rect(0.0, cloth_top + 2.0, w, 1.0);
        ctx.fill_rect(0.0, cloth_top + 5.0, w, 1.0);
    } else {
        ctx.set_fill_style_str("rgba(0, 0, 0, 0.45)");
        ctx.fill_rect(x0, y + row.shelf_h, x1 - x0, 1.0);
    }
}

fn particle_color(c: ParticleColor) -> &'static str {
    match c {
        ParticleColor::White => pal::WHITE,
        ParticleColor::Gray => pal::GRAY,
        ParticleColor::Confetti(i) => pal::CONFETTI[usize::from(i) % pal::CONFETTI.len()],
    }
}

fn draw_effects(ctx: &Ctx, state: &GameState) {
    let fx = &state.effects;

    ctx.set_fill_style_str(pal::BLACK);
    for hole in &fx.holes {
        ctx.set_global_alpha(1.0 - hole.t / HOLE_SEC);
        ctx.fill_rect(hole.x.floor() - 1.0, hole.y.floor() - 1.0, 2.0, 2.0);
    }

    for p in &fx.particles {
        ctx.set_global_alpha(p.alpha());
        ctx.set_fill_style_str(particle_color(p.color));
        ctx.fill_rect(p.x.floor(), p.y.floor(), p.size, p.size);
    }
    ctx.set_global_alpha(1.0);

    for s in &fx.stars {
        let (x, y) = s.pos();
        font::draw_text_outlined(ctx, "*", x, y, 1.0, pal::YELLOW, pal::BLACK);
    }

    for p in &fx.popups {
        let k = p.t / p.dur;
        ctx.set_global_alpha(if k < 0.5 { 1.0 } else { 2.0 * (1.0 - k) });
        let (color, scale) = match p.kind {
            PopupKind::Penalty => (pal::RED, 1.0),
            PopupKind::Good => (pal::YELLOW, 2.0),
        };
        font::draw_text_outlined(ctx, p.text, p.x, p.y - 10.0 * k, scale, color, pal::BLACK);
    }
    ctx.set_global_alpha(1.0);
}

/// 中心 `(x, y)` に、腕の長さ `len` の十字を描く。中心は空ける。
fn draw_crosshair(ctx: &Ctx, x: f64, y: f64, len: f64) {
    let (x, y) = (x.floor(), y.floor());
    let arms = [
        (x - 1.0 - len, y, len, 1.0),
        (x + 2.0, y, len, 1.0),
        (x, y - 1.0 - len, 1.0, len),
        (x, y + 2.0, 1.0, len),
    ];
    ctx.set_fill_style_str(pal::BLACK);
    for (ax, ay, aw, ah) in arms {
        ctx.fill_rect(ax - 1.0, ay - 1.0, aw + 2.0, ah + 2.0);
    }
    ctx.set_fill_style_str(pal::WHITE);
    for (ax, ay, aw, ah) in arms {
        ctx.fill_rect(ax, ay, aw, ah);
    }
}
