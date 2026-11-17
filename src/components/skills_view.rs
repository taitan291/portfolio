use leptos::prelude::*;

use crate::components::cards::skill_card::SkillCard;
use crate::components::cards::stack_style::StackStyle;
use crate::components::layout::section::Section;
use crate::components::ui::select::{Select, SelectOption};
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

    let rows = move || {
        let cfg = SortConfig::from_mode(mode.get(), &checked.get(), key.get(), order.get());
        build_rows(SKILLS, &cfg)
    };

    let actions = ViewFn::from(move || {
        view! {
            <Select
                options=mode_options.clone()
                value=mode
                on_change=Callback::new(move |m| set_mode.set(m))
                label="表示モード"
            />
        }
    });

    view! {
        <Section title="Skills" id=SKILLS_ID actions=actions>
            <div class="mb-6 space-y-4">
                <Show when=move || mode.get() == Mode::Custom>
                    <div class="space-y-3 rounded-xl border border-site-border p-4">
                        <fieldset class="flex flex-wrap gap-x-4 gap-y-2">
                            <legend class="mb-2 text-sm text-site-muted">"スタック"</legend>
                            {Stack::ALL.into_iter().map(|s| view! {
                                <label class=format!("flex items-center gap-2 text-sm {}", s.text_class())>
                                    <input
                                        type="checkbox"
                                        prop:checked=move || checked.get().contains(&s)
                                        on:change=move |ev| {
                                            let on = event_target_checked(&ev);
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
                        <div class="flex flex-wrap gap-4">
                            <Select
                                options=key_options.clone()
                                value=key
                                on_change=Callback::new(move |k| set_key.set(k))
                                label="並べ替えキー"
                            />
                            <Select
                                options=order_options.clone()
                                value=order
                                on_change=Callback::new(move |o| set_order.set(o))
                                label="並べ替え順序"
                            />
                        </div>
                    </div>
                </Show>
            </div>
            <Show
                when=move || !rows().is_empty()
                fallback=|| view! { <p class="text-site-muted">"スタックを選択してください"</p> }
            >
                <div class="space-y-8">
                    {move || rows().into_iter().map(|row| {
                        let last = row.stacks.len().saturating_sub(1);
                        view! {
                            <div>
                                <h3 class="mb-3 text-lg font-medium">
                                    {row.stacks.iter().enumerate().map(|(i, s)| view! {
                                        <span class=s.text_class()>{s.label()}</span>
                                        {(i < last).then(|| view! { <span class="text-site-muted">" / "</span> })}
                                    }).collect_view()}
                                </h3>
                                <div class="skills-grid grid gap-3 lg:gap-4">
                                    {row.entries.into_iter().map(|e| view! {
                                        <SkillCard skill=e.skill color_stack=e.stack />
                                    }).collect_view()}
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </Show>
        </Section>
    }
}
