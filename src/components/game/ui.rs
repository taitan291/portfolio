//! DOM オーバーレイ（Title / Countdown / HUD / Paused / Result）。設計書 12 章。

use super::component::{clear_ranking, GameHandle, UiSignals};
use super::engine::config::{PENALTY_SEC, SHOW_NEXT_GLYPH};
use super::engine::ranking::RANKING_LEN;
use super::engine::sequence::SEQUENCE;
use super::engine::state::{CountLabel, Phase};
use leptos::prelude::*;

const BTN_BASE: &str = "pointer-events-auto inline-flex min-h-12 items-center justify-center border-2 border-black px-6 py-2 text-lg font-bold tracking-widest shadow-[4px_4px_0_0_#000] transition-transform active:translate-x-[2px] active:translate-y-[2px] active:shadow-[2px_2px_0_0_#000]";
const BTN_PRIMARY: &str = "bg-[#d8344c] text-white hover:bg-[#e8506a]";
const BTN_SECONDARY: &str = "bg-[#fbf1da] text-[#0b0d1a] hover:bg-white";

/// ミュート切替ボタン。
#[component]
fn MuteButton(ui: UiSignals, handle: GameHandle) -> impl IntoView {
    view! {
        <button
            type="button"
            class="pointer-events-auto flex h-10 w-10 items-center justify-center border-2 border-white/70 bg-black/40 hover:bg-black/70"
            aria-label=move || if ui.muted.get() { "音を出す" } else { "ミュートする" }
            aria-pressed=move || ui.muted.get().to_string()
            on:click=move |_| handle.toggle_mute()
        >
            <svg viewBox="0 0 16 16" class="h-5 w-5" fill="currentColor" aria-hidden="true">
                <path d="M2 6h3l4-3v10l-4-3H2z" />
                {move || {
                    if ui.muted.get() {
                        view! { <path d="M11 5l4 6M15 5l-4 6" stroke="currentColor" stroke-width="1.6" fill="none" /> }
                            .into_any()
                    } else {
                        view! { <path d="M11 5.5q2 2.5 0 5M12.8 3.8q3.4 4.2 0 8.4" stroke="currentColor" stroke-width="1.4" fill="none" /> }
                            .into_any()
                    }
                }}
            </svg>
        </button>
    }
}

/// 一時停止メニューを開くボタン（Esc と同じ）。プレイ中だけ押せる。
#[component]
fn PauseButton(ui: UiSignals, handle: GameHandle) -> impl IntoView {
    let disabled = move || ui.phase.get() != Phase::Playing;
    view! {
        <button
            type="button"
            class="pointer-events-auto flex h-10 w-10 items-center justify-center border-2 border-white/70 bg-black/40 hover:bg-black/70 disabled:opacity-40"
            aria-label="一時停止メニューを開く (Esc)"
            title="一時停止 (Esc)"
            disabled=disabled
            on:click=move |_| handle.pause()
        >
            <svg viewBox="0 0 16 16" class="h-5 w-5" fill="currentColor" aria-hidden="true">
                <path d="M4 3h3v10H4zM9 3h3v10H9z" />
            </svg>
        </button>
    }
}

/// 進捗表示。撃った文字は塗り、次の文字は点滅枠。`404` を撃ち終えた直後のセットは光らせる。
#[component]
fn ProgressBar(ui: UiSignals) -> impl IntoView {
    const GROUPS: [(usize, usize); 4] = [(0, 3), (3, 6), (6, 9), (9, 17)];
    move || {
        let progress = ui.progress.get();
        let portrait = ui.portrait.get();
        let group = |(start, end): (usize, usize)| {
            let flash = progress == end && end <= 9;
            let boxes = (start..end)
                .map(|i| {
                    let state = if i < progress {
                        "border-[#f5d45e] bg-[#f5d45e] text-[#0b0d1a]"
                    } else if i == progress {
                        "animate-pulse border-white bg-white/15 text-white"
                    } else {
                        "border-white/30 text-white/45"
                    };
                    view! {
                        <span class=format!(
                            "inline-flex h-8 w-7 items-center justify-center border-2 text-lg font-bold sm:h-12 sm:w-10 sm:text-2xl {state}",
                        )>{SEQUENCE[i].as_char().to_string()}</span>
                    }
                })
                .collect_view();
            view! {
                <span class=if flash { "game-flash flex gap-0.5 rounded-sm" } else { "flex gap-0.5 rounded-sm" }>
                    {boxes}
                </span>
            }
        };
        view! {
            <div class="flex flex-wrap items-center justify-center gap-x-3 gap-y-1.5 drop-shadow-[2px_2px_0_#000]" aria-hidden="true">
                <span class="flex gap-2 sm:gap-3">{group(GROUPS[0])} {group(GROUPS[1])} {group(GROUPS[2])}</span>
                <span class=if portrait { "flex basis-full justify-center" } else { "flex" }>{group(GROUPS[3])}</span>
            </div>
        }
    }
}

/// プレイ中の HUD（タイム・進捗・ミュート）。
#[component]
pub fn Hud(ui: UiSignals, handle: GameHandle) -> impl IntoView {
    let shown = move || {
        matches!(
            ui.phase.get(),
            Phase::Playing | Phase::Paused | Phase::Clearing
        )
    };
    let next_char = move || {
        SEQUENCE
            .get(ui.progress.get())
            .map(|g| g.as_char().to_string())
            .unwrap_or_default()
    };
    view! {
        <Show when=shown>
            <div class="pointer-events-none absolute inset-x-0 top-0 flex flex-col gap-1 p-1.5 sm:gap-2 sm:p-3">
                <div class="flex items-start justify-between gap-2">
                    <div class="flex items-baseline gap-2 text-lg font-bold tracking-widest drop-shadow-[2px_2px_0_#000] sm:text-3xl">
                        <span>{move || format!("TIME {:.2}", ui.time.get())}</span>
                        {move || {
                            // 誤射のたびに作り直して、アニメーションを再生する。
                            let n = ui.penalty.get();
                            (n > 0).then(|| view! {
                                <span class="game-rise text-base text-[#ff5a6e] sm:text-xl">
                                    {format!("+{PENALTY_SEC:.1}")}
                                </span>
                            })
                        }}
                    </div>
                    {SHOW_NEXT_GLYPH.then(|| view! {
                        <div class="text-2xl font-bold drop-shadow-[2px_2px_0_#000]">"NEXT: " {next_char}</div>
                    })}
                    <div class="flex gap-2">
                        <PauseButton ui=ui handle=handle />
                        <MuteButton ui=ui handle=handle />
                    </div>
                </div>
                <ProgressBar ui=ui />
                <p class="sr-only" aria-live="polite">
                    {move || format!("{} / {} 文字。次は {}", ui.progress.get(), SEQUENCE.len(), next_char())}
                </p>
            </div>
        </Show>
    }
}

/// UNIX ミリ秒を `YYYY/MM/DD`（ローカル時刻）にする。
fn format_date(ms: f64) -> String {
    let d = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ms));
    format!(
        "{}/{:02}/{:02}",
        d.get_full_year(),
        d.get_month() + 1,
        d.get_date()
    )
}

/// 個人ランキング表。`highlight` の順位（0 始まり）を強調する。
/// 記録はこのブラウザの `localStorage` にだけ保存され、外部には送らない。
#[component]
fn RankingTable(
    ui: UiSignals,
    #[prop(default = None)] highlight: Option<usize>,
    /// true で「記録をリセット」ボタンを出す。
    #[prop(optional)]
    resettable: bool,
) -> impl IntoView {
    let rows = move || {
        let ranking = ui.ranking.get();
        let records = ranking.records();
        (0..RANKING_LEN)
            .map(|i| {
                let rec = records.get(i).copied();
                let row_class = if highlight == Some(i) {
                    "game-flash bg-[#f5d45e] text-[#0b0d1a]"
                } else if rec.is_some() {
                    ""
                } else {
                    "text-white/35"
                };
                let rank_class = if i == 0 && highlight != Some(0) {
                    "text-[#f5d45e]"
                } else {
                    ""
                };
                view! {
                    <li class=format!("grid grid-cols-[2.5rem_1fr_auto_auto] items-baseline gap-x-3 px-1.5 py-0.5 {row_class}")>
                        <span class=format!("font-bold {rank_class}")>{format!("{}.", i + 1)}</span>
                        {match rec {
                            Some(r) => view! {
                                <span class="text-right font-bold tabular-nums">{format!("{:.3}s", r.total)}</span>
                                <span class="tabular-nums">{format!("ミス{}", r.penalties)}</span>
                                <span class="tabular-nums opacity-80">{format_date(r.date_ms)}</span>
                            }
                            .into_any(),
                            None => view! {
                                <span class="text-right">"--.---s"</span>
                                <span>"ミス-"</span>
                                <span>"----/--/--"</span>
                            }
                            .into_any(),
                        }}
                    </li>
                }
            })
            .collect_view()
    };
    let is_empty = move || ui.ranking.with(|r| r.records().is_empty());
    let reset = move |_| {
        let ok = web_sys::window()
            .and_then(|w| {
                w.confirm_with_message("自己ランキングの記録をすべて消去しますか？")
                    .ok()
            })
            .unwrap_or(false);
        if ok {
            clear_ranking(ui);
        }
    };
    view! {
        <section
            class="w-full max-w-sm border-2 border-white/30 bg-black/40 px-3 py-3 text-left text-sm sm:max-w-md sm:text-base"
            aria-labelledby="game-ranking-title"
        >
            <h2 id="game-ranking-title" class="mb-2 text-center font-bold tracking-widest text-[#f5d45e]">
                "YOUR BEST 5"
            </h2>
            <ol class="flex flex-col gap-0.5">{rows}</ol>
            <Show
                when=is_empty
                fallback=move || {
                    resettable.then(|| view! {
                        <div class="mt-2 text-center">
                            <button
                                type="button"
                                class="pointer-events-auto text-sm text-white/60 underline underline-offset-4 hover:text-[#ff7a8a]"
                                on:click=reset
                            >
                                "記録をリセット"
                            </button>
                        </div>
                    })
                }
            >
                <p class="mt-2 text-center text-white/60">"クリアするとここに記録されます"</p>
            </Show>
            <p class="mt-1 text-center text-xs text-white/40 sm:text-sm">"※ 記録はこのブラウザ内にだけ保存されます"</p>
        </section>
    }
}
#[component]
fn TitleScreen(ui: UiSignals, handle: GameHandle, home_href: String) -> impl IntoView {
    view! {
        <div class="pointer-events-auto absolute inset-0 overflow-y-auto bg-[#0b0d1a]/70">
            <div class="absolute right-3 top-3">
                <MuteButton ui=ui handle=handle />
            </div>
            <div class="flex min-h-full flex-col items-center justify-center gap-4 px-4 py-8 text-center sm:gap-6">
                <h1 class="text-4xl font-bold tracking-widest text-[#f5d45e] drop-shadow-[3px_3px_0_#d8344c] sm:text-7xl">
                    "404 NOT FOUND"
                </h1>
                <p class="text-base sm:text-lg">
                    "ページが見つかりません。代わりに 4・0・4… を撃ち抜け！"
                </p>
                <div class="flex w-full flex-col items-center gap-3 sm:w-auto sm:flex-row sm:items-stretch sm:gap-4">
                    <ul class="w-full max-w-sm space-y-1 border-2 border-white/30 bg-black/40 px-4 py-3 text-left text-sm leading-relaxed sm:max-w-md sm:text-base">
                        <li>"▶ 「4 0 4」を 3 回 → 「N O T F O U N D」の順に撃て"</li>
                        <li>{format!("▶ 違う的を撃つと +{PENALTY_SEC:.1} 秒")}</li>
                        <li>
                            "▶ "
                            <span class="text-[#ff7a8a]">"0（ゼロ）は斜線入り・赤い的"</span>
                            "、"
                            <span class="text-[#8fb4ff]">"O（オー）は丸・青い的"</span>
                        </li>
                    </ul>
                    <RankingTable ui=ui resettable=true />
                </div>
                <div class="flex flex-col items-center gap-3">
                    <button
                        type="button"
                        class=format!("{BTN_BASE} {BTN_PRIMARY} min-w-48 text-2xl")
                        on:click=move |_| handle.start()
                    >
                        "START"
                    </button>
                    <a href=home_href class="pointer-events-auto text-base underline underline-offset-4 hover:text-[#f5d45e]">
                        "トップへ戻る"
                    </a>
                </div>
            </div>
        </div>
    }
}

#[component]
fn CountdownView(ui: UiSignals) -> impl IntoView {
    move || {
        ui.count.get().map(|label| {
            let text = match label {
                CountLabel::Num(n) => n.to_string(),
                CountLabel::Go => "GO!".to_string(),
            };
            // 数字が変わるたびに要素を作り直し、拡大アニメーションを再生する。
            view! {
                <div class="pointer-events-none absolute inset-0 flex items-center justify-center" aria-live="assertive">
                    <span class="game-pop text-7xl font-bold text-[#f5d45e] drop-shadow-[4px_4px_0_#000] sm:text-9xl">
                        {text}
                    </span>
                </div>
            }
        })
    }
}

/// 一時停止メニュー（再開・もう一度・ホームに戻る）。
#[component]
fn PausedView(handle: GameHandle, home_href: String) -> impl IntoView {
    let resume_ref = NodeRef::<leptos::html::Button>::new();
    // 開いたらすぐキーボード操作できるよう「再開」にフォーカスする。
    Effect::new(move |_| {
        if let Some(b) = resume_ref.get() {
            let _ = b.focus();
        }
    });
    view! {
        <div
            class="pointer-events-auto absolute inset-0 overflow-y-auto bg-black/75"
            role="dialog"
            aria-modal="true"
            aria-labelledby="game-paused-title"
        >
            <div class="flex min-h-full flex-col items-center justify-center gap-4 px-4 py-8 text-center sm:gap-6">
                <h2 id="game-paused-title" class="text-4xl font-bold tracking-widest text-[#f5d45e] drop-shadow-[3px_3px_0_#d8344c] sm:text-7xl">
                    "PAUSED"
                </h2>
                <div class="flex w-full max-w-sm flex-col gap-3">
                    <button
                        node_ref=resume_ref
                        type="button"
                        class=format!("{BTN_BASE} {BTN_PRIMARY} w-full")
                        on:click=move |_| handle.resume()
                    >
                        "再開"
                    </button>
                    <button
                        type="button"
                        class=format!("{BTN_BASE} {BTN_SECONDARY} w-full")
                        on:click=move |_| handle.start()
                    >
                        "もう一度"
                    </button>
                    <a href=home_href class=format!("{BTN_BASE} {BTN_SECONDARY} w-full")>
                        "ホームに戻る"
                    </a>
                </div>
                <p class="text-sm text-white/70 sm:text-base">"Esc キーでも再開できます"</p>
            </div>
        </div>
    }
}

#[component]
fn ResultView(ui: UiSignals, handle: GameHandle, home_href: String) -> impl IntoView {
    move || {
        let home_href = home_href.clone();
        ui.result.get().map(|r| {
            view! {
                <div class="pointer-events-auto absolute inset-0 overflow-y-auto bg-[#0b0d1a]/75">
                    <div class="flex min-h-full flex-col items-center justify-center gap-3 px-4 py-8 text-center sm:gap-5">
                        <h2 class="game-pop text-6xl font-bold tracking-widest text-[#f5d45e] drop-shadow-[3px_3px_0_#d8344c] sm:text-8xl">
                            "CLEAR!"
                        </h2>
                        <p class="text-4xl font-bold sm:text-6xl">
                            "TIME " {format!("{:.3}", r.total)} <span class="text-xl sm:text-3xl">" 秒"</span>
                        </p>
                        <p class="text-sm text-white/80 sm:text-base">
                            {format!(
                                "実時間 {:.3} 秒 ＋ ミス {} 回 (+{:.1} 秒)",
                                r.real,
                                r.penalties,
                                r.penalty_sec(),
                            )}
                        </p>
                        {r.rank.map(|rank| {
                            let label = if rank == 0 {
                                "NEW RECORD!".to_string()
                            } else {
                                format!("自己ランキング {} 位", rank + 1)
                            };
                            view! {
                                <p class="game-flash border-2 border-[#f5d45e] px-3 py-1 text-base font-bold tracking-widest text-[#f5d45e] sm:text-xl">
                                    {label}
                                </p>
                            }
                        })}
                        <RankingTable ui=ui highlight=r.rank />
                        <div class="mt-2 flex w-full max-w-sm flex-col gap-3">
                            <button
                                type="button"
                                class=format!("{BTN_BASE} {BTN_PRIMARY} w-full")
                                on:click=move |_| handle.start()
                            >
                                "もう一度"
                            </button>
                            <button
                                type="button"
                                class=format!("{BTN_BASE} {BTN_SECONDARY} w-full")
                                on:click=move |_| handle.back_to_title()
                            >
                                "スタート画面へ"
                            </button>
                            <a href=home_href class=format!("{BTN_BASE} {BTN_SECONDARY} w-full")>
                                "元のページへ戻る"
                            </a>
                        </div>
                    </div>
                </div>
            }
        })
    }
}

/// フェーズごとのオーバーレイ。
#[component]
pub fn Overlay(ui: UiSignals, handle: GameHandle, home_href: String) -> impl IntoView {
    view! {
        <div class="pointer-events-none absolute inset-0">
            {move || match ui.phase.get() {
                Phase::Title => view! { <TitleScreen ui=ui handle=handle home_href=home_href.clone() /> }.into_any(),
                Phase::Countdown | Phase::Playing => view! { <CountdownView ui=ui /> }.into_any(),
                Phase::Paused => view! { <PausedView handle=handle home_href=home_href.clone() /> }.into_any(),
                Phase::Clearing => ().into_any(),
                Phase::Result => view! { <ResultView ui=ui handle=handle home_href=home_href.clone() /> }.into_any(),
            }}
        </div>
    }
}
