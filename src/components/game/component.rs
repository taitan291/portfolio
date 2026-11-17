//! Leptos コンポーネント本体（設計書 15 章）。
//!
//! 毎フレーム変わる状態は `Engine` が `RefCell` で持ち、UI に関係する値だけをシグナルで公開する。
//! rAF ループ・window/document のリスナー・`AudioContext` はアンマウント時に必ず解放する。

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use leptos::html::{Canvas, Div};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    Event, EventTarget, HtmlCanvasElement, HtmlElement, KeyboardEvent, PointerEvent,
    ResizeObserver, VisibilityState,
};

use super::engine::config::{HUD_UPDATE_MS, PENALTY_SEC};
use super::engine::layout::{layout_for, Orientation};
use super::engine::ranking::{Ranking, Record};
use super::engine::state::{CountLabel, GameEvent, GameState, Phase, ShotResult};
use super::platform::audio::{Audio, Sfx};
use super::platform::input::{to_logical, FireLimiter};
use super::platform::raf::RafLoop;
use super::platform::renderer::Renderer;
use super::platform::storage;
use super::ui::{Hud, Overlay};

/// リザルト画面の表示内容。
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ResultInfo {
    /// ペナルティ込みのタイム（秒）。
    pub total: f64,
    /// 実時間（秒）。
    pub real: f64,
    /// 誤射の回数。
    pub penalties: u32,
    /// 個人ランキングでの順位（0 始まり）。圏外なら `None`。
    pub rank: Option<usize>,
}

impl ResultInfo {
    pub fn penalty_sec(&self) -> f64 {
        f64::from(self.penalties) * PENALTY_SEC
    }
}

/// UI 用のシグナル。
#[derive(Clone, Copy)]
pub struct UiSignals {
    pub phase: RwSignal<Phase>,
    pub count: RwSignal<Option<CountLabel>>,
    pub time: RwSignal<f64>,
    pub progress: RwSignal<usize>,
    pub penalty: RwSignal<u32>,
    pub result: RwSignal<Option<ResultInfo>>,
    pub muted: RwSignal<bool>,
    pub portrait: RwSignal<bool>,
    /// 個人ランキング（このブラウザの `localStorage` にだけ保存）。
    pub ranking: RwSignal<Ranking>,
}

impl UiSignals {
    fn new(muted: bool) -> Self {
        Self {
            phase: RwSignal::new(Phase::Title),
            count: RwSignal::new(None),
            time: RwSignal::new(0.0),
            progress: RwSignal::new(0),
            penalty: RwSignal::new(0),
            result: RwSignal::new(None),
            muted: RwSignal::new(muted),
            portrait: RwSignal::new(false),
            ranking: RwSignal::new(storage::load_ranking()),
        }
    }
}

/// 値が変わったときだけ更新する（毎フレーム呼んでも再描画を起こさない）。
fn set_if_changed<T: PartialEq + Send + Sync + 'static>(sig: RwSignal<T>, value: T) {
    if sig.with_untracked(|v| *v != value) {
        sig.set(value);
    }
}

fn perf_now() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map_or_else(js_sys::Date::now, |p| p.now())
}

fn random_seed() -> u64 {
    let r = (js_sys::Math::random() * (1u64 << 53) as f64) as u64;
    r ^ (js_sys::Date::now() as u64).rotate_left(29)
}

pub struct Engine {
    state: RefCell<GameState>,
    renderer: RefCell<Renderer>,
    audio: RefCell<Audio>,
    limiter: RefCell<FireLimiter>,
    ui: UiSignals,
    root: HtmlElement,
    stage: HtmlElement,
    last_frame_ms: Cell<Option<f64>>,
    last_hud_ms: Cell<f64>,
    /// 一時停止画面を描き終えたか（停止中は再描画しない）。
    paused_drawn: Cell<bool>,
    reduced_motion: bool,
}

impl Engine {
    fn frame(&self, ts: f64) {
        let dt = self
            .last_frame_ms
            .replace(Some(ts))
            .map_or(0.0, |prev| (ts - prev) / 1000.0);
        let paused = self.state.borrow().phase == Phase::Paused;
        if paused && self.paused_drawn.get() {
            return;
        }
        let events = self.state.borrow_mut().update(ts, dt);
        self.handle_events(&events);
        self.sync_ui(ts);
        self.render();
        self.paused_drawn.set(paused);
    }

    fn render(&self) {
        self.renderer
            .borrow_mut()
            .draw(&self.state.borrow(), self.reduced_motion);
    }

    fn handle_events(&self, events: &[GameEvent]) {
        for e in events {
            match *e {
                GameEvent::CountdownTick(_) => self.audio.borrow().play(Sfx::Count),
                GameEvent::Go => self.audio.borrow().play(Sfx::Go),
                GameEvent::PhaseChanged(p) => {
                    self.paused_drawn.set(false);
                    if p == Phase::Result {
                        let now = perf_now();
                        let s = self.state.borrow();
                        let total = s.display_time(now);
                        let mut rank = None;
                        self.ui.ranking.update(|r| {
                            rank = r.insert(Record {
                                total,
                                penalties: s.penalty_count,
                                date_ms: js_sys::Date::now(),
                            });
                            if rank.is_some() {
                                storage::save_ranking(r);
                            }
                        });
                        self.ui.result.set(Some(ResultInfo {
                            total,
                            real: s.elapsed(now),
                            penalties: s.penalty_count,
                            rank,
                        }));
                    } else if matches!(p, Phase::Title | Phase::Countdown) {
                        self.ui.result.set(None);
                    }
                    set_if_changed(self.ui.phase, p);
                }
            }
        }
    }

    fn sync_ui(&self, now: f64) {
        let mut s = self.state.borrow_mut();
        if s.phase != Phase::Playing {
            s.aim = None;
        }
        set_if_changed(self.ui.count, s.count_label());
        set_if_changed(self.ui.progress, s.progress);
        set_if_changed(self.ui.penalty, s.penalty_count);
        // タイムは約 10Hz で更新する。止まっているときは正確な値を出す。
        let running = s.phase == Phase::Playing;
        if !running || now - self.last_hud_ms.get() >= HUD_UPDATE_MS {
            self.last_hud_ms.set(now);
            set_if_changed(self.ui.time, s.display_time(now));
        }
    }

    fn canvas(&self) -> HtmlCanvasElement {
        self.renderer.borrow().canvas().clone()
    }

    fn logical(&self, ev: &PointerEvent) -> Option<(f64, f64)> {
        let layout = self.state.borrow().layout;
        to_logical(
            &self.canvas(),
            f64::from(ev.client_x()),
            f64::from(ev.client_y()),
            layout.w(),
            layout.h(),
        )
    }

    pub fn shoot(&self, ev: &PointerEvent) {
        let now = perf_now();
        if self.state.borrow().phase != Phase::Playing
            || !self.limiter.borrow_mut().allow(ev.pointer_id(), now)
        {
            return;
        }
        let Some((x, y)) = self.logical(ev) else {
            return;
        };
        let touch = ev.pointer_type() == "touch";
        let result = self.state.borrow_mut().shoot(x, y, touch, now);
        let audio = self.audio.borrow();
        audio.play(Sfx::Shot);
        match result {
            Some(ShotResult::Correct { set, cleared }) => {
                audio.play(Sfx::Hit);
                if cleared {
                    audio.play(Sfx::Clear);
                } else if set.is_some() {
                    audio.play(Sfx::SetDone);
                }
            }
            Some(ShotResult::Wrong) => audio.play(Sfx::Wrong),
            Some(ShotResult::Miss) | None => {}
        }
        drop(audio);
        self.handle_events(&[GameEvent::PhaseChanged(self.state.borrow().phase)]);
        self.sync_ui(now);
    }

    pub fn aim(&self, ev: &PointerEvent) {
        if ev.pointer_type() != "mouse" || self.state.borrow().phase != Phase::Playing {
            return;
        }
        let pos = self.logical(ev);
        self.state.borrow_mut().aim = pos;
    }

    pub fn clear_aim(&self) {
        self.state.borrow_mut().aim = None;
    }

    pub fn start(&self) {
        // ユーザー操作の中で AudioContext を作る（自動再生制限の対策）。
        self.audio.borrow_mut().unlock();
        let events = self.state.borrow_mut().start_countdown();
        self.handle_events(&events);
        self.sync_ui(perf_now());
    }

    pub fn back_to_title(&self) {
        let events = self.state.borrow_mut().back_to_title();
        self.handle_events(&events);
        self.sync_ui(perf_now());
    }

    pub fn pause(&self) {
        let events = self.state.borrow_mut().pause(perf_now());
        self.handle_events(&events);
    }

    /// Esc 用。プレイ中なら一時停止、一時停止中なら再開する。
    pub fn toggle_pause(&self) {
        let paused = self.state.borrow().phase == Phase::Paused;
        if paused {
            self.resume();
        } else {
            self.pause();
        }
    }

    pub fn resume(&self) {
        let now = perf_now();
        let events = self.state.borrow_mut().resume(now);
        self.handle_events(&events);
        self.last_frame_ms.set(None);
        self.sync_ui(now);
    }

    pub fn toggle_mute(&self) {
        let muted = !self.ui.muted.get_untracked();
        let mut audio = self.audio.borrow_mut();
        audio.set_muted(muted);
        if !muted {
            audio.unlock();
        }
        self.ui.muted.set(muted);
    }

    fn on_visibility(&self) {
        let hidden = web_sys::window()
            .and_then(|w| w.document())
            .is_some_and(|d| d.visibility_state() == VisibilityState::Hidden);
        if hidden {
            let events = self.state.borrow_mut().on_hidden(perf_now());
            self.handle_events(&events);
        }
        self.last_frame_ms.set(None);
    }

    /// 親要素のサイズから向きと Canvas の表示サイズを決める（設計書 16 章）。
    fn on_resize(&self) {
        let rect = self.root.get_bounding_client_rect();
        let (w, h) = (rect.width(), rect.height());
        if w <= 0.0 || h <= 0.0 {
            return;
        }
        let o = Orientation::from_size(w, h);
        let events = self.state.borrow_mut().set_orientation(o, perf_now());
        self.handle_events(&events);
        let layout = layout_for(o);
        self.renderer.borrow_mut().set_layout(layout);
        set_if_changed(self.ui.portrait, o == Orientation::Portrait);

        // 2 倍以上の整数倍で収まるなら整数倍、そうでなければ比率を保って fit。
        let fit = (w / layout.w()).min(h / layout.h());
        let scale = if fit.floor() >= 2.0 { fit.floor() } else { fit };
        let style = self.stage.style();
        let _ = style.set_property("width", &format!("{}px", (layout.w() * scale).floor()));
        let _ = style.set_property("height", &format!("{}px", (layout.h() * scale).floor()));
        self.paused_drawn.set(false);
        self.render();
    }
}

type Listener = (EventTarget, &'static str, Closure<dyn FnMut(Event)>);

/// アンマウント時に解放するもの一式。
struct Resources {
    engine: Rc<Engine>,
    raf: RafLoop,
    listeners: Vec<Listener>,
    observer: Option<(ResizeObserver, Closure<dyn FnMut()>)>,
}

impl Resources {
    fn create(
        root: HtmlElement,
        stage: HtmlElement,
        canvas: HtmlCanvasElement,
        ui: UiSignals,
    ) -> Option<Self> {
        let window = web_sys::window()?;
        let document = window.document()?;
        let rect = root.get_bounding_client_rect();
        let orientation = Orientation::from_size(rect.width(), rect.height());
        let renderer = Renderer::new(canvas, layout_for(orientation))?;
        let reduced_motion = window
            .match_media("(prefers-reduced-motion: reduce)")
            .ok()
            .flatten()
            .is_some_and(|m| m.matches());

        let engine = Rc::new(Engine {
            state: RefCell::new(GameState::new(random_seed(), orientation)),
            renderer: RefCell::new(renderer),
            audio: RefCell::new(Audio::new(ui.muted.get_untracked())),
            limiter: RefCell::new(FireLimiter::default()),
            ui,
            root: root.clone(),
            stage,
            last_frame_ms: Cell::new(None),
            last_hud_ms: Cell::new(0.0),
            paused_drawn: Cell::new(false),
            reduced_motion,
        });
        engine.on_resize();

        let mut listeners = Vec::new();
        let mut listen = |target: EventTarget, name: &'static str, f: Box<dyn FnMut(Event)>| {
            let closure = Closure::wrap(f);
            if target
                .add_event_listener_with_callback(name, closure.as_ref().unchecked_ref())
                .is_ok()
            {
                listeners.push((target, name, closure));
            }
        };
        let e = Rc::clone(&engine);
        listen(
            document.into(),
            "visibilitychange",
            Box::new(move |_| e.on_visibility()),
        );
        let e = Rc::clone(&engine);
        listen(
            window.clone().into(),
            "keydown",
            Box::new(move |ev: Event| {
                if ev
                    .dyn_ref::<KeyboardEvent>()
                    .is_some_and(|k| k.key() == "Escape")
                {
                    e.toggle_pause();
                }
            }),
        );
        // ResizeObserver が使えない環境向けの保険。
        let e = Rc::clone(&engine);
        listen(window.into(), "resize", Box::new(move |_| e.on_resize()));

        let e = Rc::clone(&engine);
        let on_resize = Closure::<dyn FnMut()>::new(move || e.on_resize());
        let observer = ResizeObserver::new(on_resize.as_ref().unchecked_ref())
            .ok()
            .map(|obs| {
                obs.observe(&root);
                (obs, on_resize)
            });

        let e = Rc::clone(&engine);
        let raf = RafLoop::start(move |ts| e.frame(ts));

        Some(Self {
            engine,
            raf,
            listeners,
            observer,
        })
    }

    fn dispose(self) {
        self.raf.stop();
        for (target, name, closure) in &self.listeners {
            let _ =
                target.remove_event_listener_with_callback(name, closure.as_ref().unchecked_ref());
        }
        if let Some((obs, _)) = &self.observer {
            obs.disconnect();
        }
        self.engine.audio.borrow_mut().close();
    }
}

/// UI からゲームを操作するためのハンドル（`Copy`）。
#[derive(Clone, Copy)]
pub struct GameHandle(StoredValue<Option<Resources>, LocalStorage>);

impl GameHandle {
    fn new() -> Self {
        Self(StoredValue::new_local(None))
    }

    fn engine(&self) -> Option<Rc<Engine>> {
        self.0
            .try_with_value(|r| r.as_ref().map(|r| Rc::clone(&r.engine)))
            .flatten()
    }

    fn with(&self, f: impl FnOnce(&Engine)) {
        if let Some(e) = self.engine() {
            f(&e);
        }
    }

    pub fn start(&self) {
        self.with(Engine::start);
    }

    pub fn back_to_title(&self) {
        self.with(Engine::back_to_title);
    }

    pub fn pause(&self) {
        self.with(Engine::pause);
    }

    pub fn resume(&self) {
        self.with(Engine::resume);
    }

    pub fn toggle_mute(&self) {
        self.with(Engine::toggle_mute);
    }
}

/// 個人ランキングを消去する。
pub fn clear_ranking(ui: UiSignals) {
    storage::clear_ranking();
    ui.ranking.set(Ranking::default());
}

/// 404 ページ用の射的ゲーム。親要素いっぱい（高さは `100dvh`）に表示する。
#[component]
pub fn ShootingGallery(
    /// 「元のページへ戻る」の遷移先。
    #[prop(into, default = "/".to_string())]
    home_href: String,
    /// true で起動時からミュート。
    #[prop(default = false)]
    start_muted: bool,
) -> impl IntoView {
    let root_ref = NodeRef::<Div>::new();
    let stage_ref = NodeRef::<Div>::new();
    let canvas_ref = NodeRef::<Canvas>::new();
    let ui = UiSignals::new(start_muted);
    let handle = GameHandle::new();

    // マウント後に一度だけ初期化する。
    Effect::new(move |_| {
        let (Some(root), Some(stage), Some(canvas)) =
            (root_ref.get(), stage_ref.get(), canvas_ref.get())
        else {
            return;
        };
        if handle.engine().is_some() {
            return;
        }
        if let Some(res) = Resources::create(root.into(), stage.into(), canvas, ui) {
            handle.0.set_value(Some(res));
        }
    });

    // SPA のページ遷移でループやリスナーが残らないよう、必ず解放する。
    on_cleanup(move || {
        if let Some(res) = handle.0.try_update_value(Option::take).flatten() {
            res.dispose();
        }
    });

    let playing = move || ui.phase.get() == Phase::Playing;

    view! {
        <div
            node_ref=root_ref
            class="relative flex h-[100dvh] w-full select-none items-center justify-center overflow-hidden bg-game-black font-mono text-white"
        >
            <div node_ref=stage_ref class="relative h-[180px] w-[320px]">
                <canvas
                    node_ref=canvas_ref
                    width="320"
                    height="180"
                    aria-label="射的ゲームのフィールド。4 0 4 を 3 回、続けて N O T F O U N D の順に的を撃つ"
                    class="block h-full w-full touch-none [image-rendering:pixelated]"
                    class:cursor-none=playing
                    on:pointerdown=move |ev: PointerEvent| {
                        ev.prevent_default();
                        handle.with(|e| e.shoot(&ev));
                    }
                    on:pointermove=move |ev: PointerEvent| handle.with(|e| e.aim(&ev))
                    on:pointerleave=move |_: PointerEvent| handle.with(Engine::clear_aim)
                    on:contextmenu=|ev: web_sys::MouseEvent| ev.prevent_default()
                >
                    "このブラウザでは Canvas を表示できません。"
                </canvas>
                <Hud ui=ui handle=handle />
            </div>
            <Overlay ui=ui handle=handle home_href=home_href />
        </div>
    }
}
