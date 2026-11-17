use leptos::prelude::*;

use crate::components::cards::stack_style::StackStyle;
use crate::components::ui::modal::Modal;
use crate::data::skills::Skill;
use crate::data::types::{Label, Stack};

/// スキルのカード。クリックで習熟度の詳細をモーダル表示する。
#[component]
pub fn SkillCard(
    skill: &'static Skill,
    /// カードの色に使うスタック。
    color_stack: Stack,
) -> impl IntoView {
    let Skill {
        name,
        image_url,
        stacks,
        credit,
        ..
    } = *skill;

    view! {
        <Modal
            title=name
            trigger=move || view! {
                <span class=format!("mx-auto flex h-24 w-24 items-center justify-center rounded-xl {}", color_stack.bg_class())>
                    <img src=image_url alt="" class="h-20 w-20 object-contain" />
                </span>
                <span class="mt-3 block break-words text-xl font-medium">{name}</span>
            }
        >
            <div class="flex flex-col gap-5 sm:flex-row sm:items-start">
                <div class=format!("self-center shrink-0 rounded-xl p-2 sm:self-start {}", color_stack.bg_class())>
                    <img src=image_url alt="" class="h-24 w-24 sm:h-40 sm:w-40 object-contain" />
                </div>
                <div class="min-w-0 flex-1 space-y-4">
                    {stacks.iter().map(|s| view! {
                        <div>
                            <div class="mb-2 flex flex-wrap items-center justify-between gap-2">
                                <span class=format!("rounded border px-2 py-1 text-sm {} {} {}", s.stack.bg_class(), s.stack.text_class(), s.stack.border_class())>
                                    {s.stack.label()}
                                </span>
                                <span class=format!("text-sm {}", s.stack.text_class())>{s.proficiency.label()}</span>
                            </div>
                            <div class="h-2 overflow-hidden rounded-full bg-site-subtle" aria-hidden="true">
                                <div style=format!("--level:{}%;background-color:{}", s.level, s.stack.bar_color(s.level)) class="skill-bar h-full rounded-full" />
                            </div>
                        </div>
                    }).collect_view()}
                </div>
            </div>
            <p class="mt-6 border-t border-site-border pt-3 text-xs text-site-muted">{credit}</p>
        </Modal>
    }
}
