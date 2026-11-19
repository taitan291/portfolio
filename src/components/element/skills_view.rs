use leptos::prelude::*;

use super::select::{Select, SelectOption};
use super::skills_card::SkillsCard;
use crate::components::layout::section::Section;
use crate::components::reveal::reveal_delay;
use crate::data::site::SKILLS_ID;
use crate::data::skill_sort::{build_rows, Mode, SortConfig, SortKey, SortOrder};
use crate::data::skills::SKILLS;
use crate::data::types::{Label, Stack};

fn mode_option(mode: Mode) -> SelectOption<Mode> {
    match mode {
        Mode::Default => SelectOption::new(mode, "Default", ""),
        Mode::Single(s) => SelectOption::new(mode, s.label(), s.text_class()),
        Mode::Custom => SelectOption::new(mode, "Custom", ""),
    }
}

/// Skills セクション。表示モードに応じてスキルを段ごとに並べ替えて表示する。
#[component]
pub fn SkillsView() -> impl IntoView {
    let mode_options: Vec<_> = std::iter::once(Mode::Default)
        .chain(Stack::ALL.map(Mode::Single))
        .chain(std::iter::once(Mode::Custom))
        .map(mode_option)
        .collect();
    let key_options = vec![
        SelectOption::new(SortKey::Proficiency, "習熟度", ""),
        SelectOption::new(SortKey::Name, "名前", ""),
    ];
    let order_options = vec![
        SelectOption::new(SortOrder::Desc, "降順", ""),
        SelectOption::new(SortOrder::Asc, "昇順", ""),
    ];

    let (mode, set_mode) = signal(Mode::Default);
    let (checked, set_checked) = signal(Vec::<Stack>::new());
    let (key, set_key) = signal(SortKey::Proficiency);
    let (order, set_order) = signal(SortOrder::Desc);
    // 並べ替えを一度でも操作したか。スクロールでのフェードインは最初に描画したカードにしか
    // 効かないので、操作後に描き直したカードは CSS アニメーションで表示する。
    let touched = RwSignal::new(false);

    let rows = move || {
        let cfg = SortConfig::from_mode(mode.get(), &checked.get(), key.get(), order.get());
        build_rows(SKILLS, &cfg)
    };

    let actions = ViewFn::from(move || {
        view! {
            <Select
                options=mode_options.clone()
                value=mode
                on_change=Callback::new(move |m| {
                    touched.set(true);
                    set_mode.set(m);
                })
                label="表示モード"
            />
        }
    });

    view! {
        <Section
            number="02"
            title="Skills"
            tag=SKILLS_ID
            lead="これまでに触れてきた言語・ツール。カードを選ぶと詳細と関連作品を表示します。"
            actions=actions
        >
            <Show when=move || mode.get() == Mode::Custom>
                <div class="card mb-10 flex animate-fade-up flex-col gap-5 p-5 sm:p-6">
                    <fieldset class="flex flex-wrap gap-x-5 gap-y-2">
                        <legend class="mb-3 font-mono text-xs uppercase tracking-widest text-muted">"Stacks"</legend>
                        {Stack::ALL.into_iter().map(|s| view! {
                            <label class=format!("flex min-h-11 cursor-pointer items-center gap-2 font-mono text-sm {}", s.text_class())>
                                <input
                                    type="checkbox"
                                    class="h-4 w-4 accent-accent-text"
                                    prop:checked=move || checked.get().contains(&s)
                                    on:change=move |ev| {
                                        let on = event_target_checked(&ev);
                                        touched.set(true);
                                        set_checked.update(|v| {
                                            v.retain(|x| *x != s);
                                            if on { v.push(s); }
                                        });
                                    }
                                />
                                {s.label()}
                            </label>
                        }).collect_view()}
                    </fieldset>
                    <div class="flex flex-wrap gap-3">
                        <Select
                            options=key_options.clone()
                            value=key
                            on_change=Callback::new(move |k| {
                                touched.set(true);
                                set_key.set(k);
                            })
                            label="並べ替えキー"
                        />
                        <Select
                            options=order_options.clone()
                            value=order
                            on_change=Callback::new(move |o| {
                                touched.set(true);
                                set_order.set(o);
                            })
                            label="並べ替え順序"
                        />
                    </div>
                </div>
            </Show>
            <Show
                when=move || !rows().is_empty()
                fallback=|| view! { <p class="font-mono text-sm text-muted">"スタックを選択してください"</p> }
            >
                <div class="space-y-12">
                    {move || {
                        let animate = touched.get_untracked();
                        rows().into_iter().map(|row| {
                            let last = row.stacks.len().saturating_sub(1);
                            view! {
                                <div>
                                    <h3 data-reveal="" class=("is-revealed", animate) class="mb-4 font-mono text-sm">
                                        {row.stacks.iter().enumerate().map(|(i, s)| view! {
                                            <span class=s.text_class()>{s.label()}</span>
                                            {(i < last).then(|| view! { <span class="text-muted">" / "</span> })}
                                        }).collect_view()}
                                    </h3>
                                    <div class="skills-grid grid gap-4">
                                        {row.entries.into_iter().enumerate().map(|(index, e)| {
                                            // 1 行（最大 4 列）の中で遅延をずらし、後の行が遅れすぎないようにする。
                                            let style = if animate {
                                                format!("animation-delay: {}ms", index * 40)
                                            } else {
                                                reveal_delay(index % 4)
                                            };
                                            view! {
                                                <div
                                                    data-reveal=""
                                                    class=("is-revealed", animate)
                                                    class=("animate-fade-up", animate)
                                                    style=style
                                                >
                                                    <SkillsCard skill=e.skill color_stack=e.stack />
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                            }
                        }).collect_view()
                    }}
                </div>
            </Show>
        </Section>
    }
}
