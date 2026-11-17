//! パーティクル・ポップアップ・弾痕・画面揺れ（設計書 11.1）。

use super::config::MAX_PARTICLES;
use super::rng::Rng;
use std::f64::consts::TAU;

/// パーティクルの色。実際の色はプラットフォーム側のパレットで決める。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ParticleColor {
    /// 正解ヒットの破片。
    White,
    /// ミスヒットの破片。
    Gray,
    /// 紙吹雪（パレットの添字）。
    Confetti(u8),
}

#[derive(Clone, Debug)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub gravity: f64,
    pub life: f64,
    pub max_life: f64,
    pub size: f64,
    pub color: ParticleColor,
}

impl Particle {
    pub fn alpha(&self) -> f64 {
        (self.life / self.max_life).clamp(0.0, 1.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PopupKind {
    /// 誤射ペナルティ（赤）。
    Penalty,
    /// 正解・セット完了（黄）。
    Good,
}

#[derive(Clone, Debug)]
pub struct Popup {
    pub text: &'static str,
    pub x: f64,
    pub y: f64,
    pub t: f64,
    pub dur: f64,
    pub kind: PopupKind,
}

/// 正解を撃ったときに進捗表示へ飛ぶ星。
#[derive(Clone, Debug)]
pub struct Star {
    pub from: (f64, f64),
    pub to: (f64, f64),
    pub t: f64,
    pub dur: f64,
}

impl Star {
    pub fn pos(&self) -> (f64, f64) {
        let k = (self.t / self.dur).clamp(0.0, 1.0);
        let k = 1.0 - (1.0 - k).powi(2);
        (
            self.from.0 + (self.to.0 - self.from.0) * k,
            self.from.1 + (self.to.1 - self.from.1) * k,
        )
    }
}

#[derive(Clone, Debug)]
pub struct BulletHole {
    pub x: f64,
    pub y: f64,
    pub t: f64,
}

/// タッチ時にタップ位置へ一瞬出す照準。
#[derive(Clone, Debug)]
pub struct TapMark {
    pub x: f64,
    pub y: f64,
    pub t: f64,
}

pub const POPUP_SEC: f64 = 0.6;
pub const STAR_SEC: f64 = 0.45;
pub const HOLE_SEC: f64 = 0.5;
pub const TAP_MARK_SEC: f64 = 0.2;
pub const SHAKE_SEC: f64 = 0.12;
pub const SHAKE_PX: f64 = 2.0;
const CONFETTI_COUNT: usize = 40;
const SHARD_COUNT: usize = 6;

#[derive(Clone, Debug, Default)]
pub struct Effects {
    pub particles: Vec<Particle>,
    pub popups: Vec<Popup>,
    pub stars: Vec<Star>,
    pub holes: Vec<BulletHole>,
    pub taps: Vec<TapMark>,
    /// 画面揺れの残り時間。
    pub shake_left: f64,
}

impl Effects {
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    fn push_particle(&mut self, p: Particle) {
        if self.particles.len() >= MAX_PARTICLES {
            self.particles.remove(0);
        }
        self.particles.push(p);
    }

    /// 的が割れたときの破片。
    pub fn shards(&mut self, rng: &mut Rng, x: f64, y: f64, scale: f64, color: ParticleColor) {
        for _ in 0..SHARD_COUNT {
            let a = rng.range(-TAU * 0.45, -TAU * 0.05);
            let speed = rng.range(40.0, 90.0) * scale;
            let life = rng.range(0.35, 0.6);
            self.push_particle(Particle {
                x,
                y,
                vx: a.cos() * speed,
                vy: a.sin() * speed,
                gravity: 260.0,
                life,
                max_life: life,
                size: if scale >= 0.9 { 2.0 } else { 1.0 },
                color,
            });
        }
    }

    /// クリア時の紙吹雪。
    pub fn confetti(&mut self, rng: &mut Rng, width: f64) {
        for i in 0..CONFETTI_COUNT {
            let life = rng.range(1.2, 2.2);
            self.push_particle(Particle {
                x: rng.range(0.0, width),
                y: rng.range(-30.0, -2.0),
                vx: rng.range(-20.0, 20.0),
                vy: rng.range(20.0, 60.0),
                gravity: 40.0,
                life,
                max_life: life,
                size: 2.0,
                color: ParticleColor::Confetti((i % 6) as u8),
            });
        }
    }

    pub fn popup(&mut self, text: &'static str, x: f64, y: f64, kind: PopupKind) {
        self.popups.push(Popup {
            text,
            x,
            y,
            t: 0.0,
            dur: POPUP_SEC,
            kind,
        });
    }

    pub fn star(&mut self, from: (f64, f64), to: (f64, f64)) {
        self.stars.push(Star {
            from,
            to,
            t: 0.0,
            dur: STAR_SEC,
        });
    }

    pub fn hole(&mut self, x: f64, y: f64) {
        self.holes.push(BulletHole { x, y, t: 0.0 });
    }

    pub fn tap(&mut self, x: f64, y: f64) {
        self.taps.push(TapMark { x, y, t: 0.0 });
    }

    pub fn shake(&mut self) {
        self.shake_left = SHAKE_SEC;
    }

    /// 画面揺れのオフセット（整数 px）。
    pub fn shake_offset(&self, rng_phase: f64) -> (f64, f64) {
        if self.shake_left <= 0.0 {
            return (0.0, 0.0);
        }
        // 1 フレームごとに左右へ振る。
        let k = (rng_phase * 60.0).floor() as i64;
        let dx = if k % 2 == 0 { SHAKE_PX } else { -SHAKE_PX };
        let dy = if (k / 2) % 2 == 0 { 1.0 } else { -1.0 };
        (dx, dy)
    }

    pub fn update(&mut self, dt: f64) {
        for p in &mut self.particles {
            p.vy += p.gravity * dt;
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);
        for p in &mut self.popups {
            p.t += dt;
        }
        self.popups.retain(|p| p.t < p.dur);
        for s in &mut self.stars {
            s.t += dt;
        }
        self.stars.retain(|s| s.t < s.dur);
        for h in &mut self.holes {
            h.t += dt;
        }
        self.holes.retain(|h| h.t < HOLE_SEC);
        for m in &mut self.taps {
            m.t += dt;
        }
        self.taps.retain(|m| m.t < TAP_MARK_SEC);
        self.shake_left = (self.shake_left - dt).max(0.0);
    }

    /// 動いている演出が残っているか。
    #[cfg(test)]
    pub fn is_idle(&self) -> bool {
        self.particles.is_empty()
            && self.popups.is_empty()
            && self.stars.is_empty()
            && self.holes.is_empty()
            && self.taps.is_empty()
            && self.shake_left <= 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn particle_cap_and_fade() {
        let mut e = Effects::default();
        let mut rng = Rng::new(1);
        for _ in 0..30 {
            e.shards(&mut rng, 10.0, 10.0, 1.0, ParticleColor::White);
        }
        assert_eq!(e.particles.len(), MAX_PARTICLES);
        e.confetti(&mut rng, 320.0);
        assert_eq!(e.particles.len(), MAX_PARTICLES);
        for _ in 0..200 {
            e.update(0.02);
        }
        assert!(e.is_idle());
    }
}
