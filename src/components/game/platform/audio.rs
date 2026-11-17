//! Web Audio による効果音の合成（設計書 11.2）。音声ファイルは使わない。
//!
//! `AudioContext` はユーザー操作（START）を起点に作る。作れなかった場合は無音で続行する。

use wasm_bindgen::JsValue;
use web_sys::{AudioBuffer, AudioContext, AudioNode, BiquadFilterType, OscillatorType};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sfx {
    Shot,
    Hit,
    Wrong,
    Count,
    Go,
    SetDone,
    Clear,
}

const MASTER_VOLUME: f32 = 0.18;
/// ゲインを 0 に向けて指数減衰させるときの下限（0 は指定できない）。
const SILENT: f32 = 0.0001;

pub struct Audio {
    ctx: Option<AudioContext>,
    noise: Option<AudioBuffer>,
    muted: bool,
}

impl Audio {
    pub fn new(muted: bool) -> Self {
        Self {
            ctx: None,
            noise: None,
            muted,
        }
    }

    /// ユーザー操作のハンドラ内で呼ぶ。`AudioContext` を作るか、停止中なら再開する。
    pub fn unlock(&mut self) {
        if self.ctx.is_none() {
            self.ctx = AudioContext::new().ok();
            self.noise = self.ctx.as_ref().and_then(|c| make_noise(c).ok());
        }
        if let Some(ctx) = &self.ctx {
            if ctx.state() == web_sys::AudioContextState::Suspended {
                let _ = ctx.resume();
            }
        }
    }

    pub fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
    }

    pub fn close(&mut self) {
        if let Some(ctx) = self.ctx.take() {
            let _ = ctx.close();
        }
        self.noise = None;
    }

    pub fn play(&self, sfx: Sfx) {
        if self.muted {
            return;
        }
        let Some(ctx) = &self.ctx else {
            return;
        };
        // 失敗しても無音になるだけなので無視する。
        let _ = self.try_play(ctx, sfx);
    }

    fn try_play(&self, ctx: &AudioContext, sfx: Sfx) -> Result<(), JsValue> {
        use OscillatorType::{Sawtooth, Square};
        let t = ctx.current_time();
        match sfx {
            Sfx::Shot => self.noise_burst(ctx, t, 0.04, 1.0)?,
            Sfx::Hit => tone(ctx, Square, 880.0, 1320.0, t, 0.08, 0.6)?,
            Sfx::Wrong => tone(ctx, Sawtooth, 220.0, 110.0, t, 0.15, 0.7)?,
            Sfx::Count => tone(ctx, Square, 660.0, 660.0, t, 0.10, 0.5)?,
            Sfx::Go => tone(ctx, Square, 990.0, 990.0, t, 0.20, 0.5)?,
            Sfx::SetDone => {
                // C-E-G のアルペジオ。
                for (i, f) in [523.25, 659.25, 783.99].into_iter().enumerate() {
                    tone(ctx, Square, f, f, t + 0.06 * i as f64, 0.06, 0.45)?;
                }
            }
            Sfx::Clear => {
                let notes = [
                    (392.00, 0.12),
                    (523.25, 0.12),
                    (659.25, 0.12),
                    (783.99, 0.14),
                    (1046.50, 0.30),
                ];
                let mut at = t;
                for (f, d) in notes {
                    tone(ctx, Square, f, f, at, d, 0.45)?;
                    at += d;
                }
            }
        }
        Ok(())
    }

    /// ノイズをハイパスに通した「パン」という発射音。
    fn noise_burst(&self, ctx: &AudioContext, at: f64, dur: f64, vol: f32) -> Result<(), JsValue> {
        let Some(buf) = &self.noise else {
            return Ok(());
        };
        let src = ctx.create_buffer_source()?;
        src.set_buffer(Some(buf));
        let filter = ctx.create_biquad_filter()?;
        filter.set_type(BiquadFilterType::Highpass);
        filter.frequency().set_value(1500.0);
        let gain = envelope(ctx, at, dur, vol)?;
        src.connect_with_audio_node(&filter)?;
        filter.connect_with_audio_node(&gain)?;
        src.start_with_when(at)?;
        Ok(())
    }
}

/// 出力先に繋がった、減衰エンベロープ付きのゲインノード。
fn envelope(ctx: &AudioContext, at: f64, dur: f64, vol: f32) -> Result<AudioNode, JsValue> {
    let gain = ctx.create_gain()?;
    let g = gain.gain();
    g.set_value_at_time(vol * MASTER_VOLUME, at)?;
    g.exponential_ramp_to_value_at_time(SILENT, at + dur)?;
    gain.connect_with_audio_node(&ctx.destination())?;
    Ok(gain.into())
}

fn tone(
    ctx: &AudioContext,
    wave: OscillatorType,
    f0: f32,
    f1: f32,
    at: f64,
    dur: f64,
    vol: f32,
) -> Result<(), JsValue> {
    let osc = ctx.create_oscillator()?;
    osc.set_type(wave);
    let freq = osc.frequency();
    freq.set_value_at_time(f0, at)?;
    if (f1 - f0).abs() > f32::EPSILON {
        freq.exponential_ramp_to_value_at_time(f1, at + dur)?;
    }
    let gain = envelope(ctx, at, dur, vol)?;
    osc.connect_with_audio_node(&gain)?;
    osc.start_with_when(at)?;
    osc.stop_with_when(at + dur + 0.02)?;
    Ok(())
}

/// 0.1 秒分のホワイトノイズ。乱数は固定 seed の xorshift。
fn make_noise(ctx: &AudioContext) -> Result<AudioBuffer, JsValue> {
    let rate = ctx.sample_rate();
    let len = (rate * 0.1) as u32;
    let buf = ctx.create_buffer(1, len, rate)?;
    let mut rng = crate::components::game::engine::rng::Rng::new(0x5EED);
    let data: Vec<f32> = (0..len)
        .map(|_| (rng.next_f64() * 2.0 - 1.0) as f32)
        .collect();
    buf.copy_to_channel(&data, 0)?;
    Ok(buf)
}
