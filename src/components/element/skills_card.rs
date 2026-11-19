use leptos::prelude::*;

use super::modal::Modal;
use super::work_thumb::WorkThumb;
use crate::data::skills::Skill;
use crate::data::types::{Label, Lang, Stack};
use crate::data::works::WORKS;

/// スキルのカード。クリックで習熟度の詳細と関連作品をモーダル表示する。
#[component]
pub fn SkillsCard(
    skill: &'static Skill,
    /// カードに表示するスタック（並べ替えの段で評価したもの）。
    color_stack: Stack,
) -> impl IntoView {
    let Skill {
        name,
        image_url,
        lang,
        stacks,
        credit,
    } = *skill;
    // カードには `color_stack` での習熟度を出す（該当がなければ先頭のスタック）。
    let shown = stacks
        .iter()
        .find(|s| s.stack == color_stack)
        .or_else(|| stacks.first())
        .copied();

    view! {
        <Modal
            title=name
            class="flex flex-col p-5"
            trigger=move || view! {
                <span class="flex items-start justify-between gap-3">
                    <span class="grid h-14 w-14 shrink-0 place-items-center rounded-xl bg-white ring-1 ring-black/5 transition-transform duration-300 group-hover:scale-105">
                        <img src=image_url alt="" width="36" height="36" loading="lazy" decoding="async" class="h-9 w-9 object-contain" />
                    </span>
                    <span class="font-mono text-xs text-muted transition duration-300 group-hover:-translate-y-0.5 group-hover:translate-x-0.5 group-hover:text-fg" aria-hidden="true">"↗"</span>
                </span>
                <span class="mt-5 block break-words text-xl font-bold tracking-tight">{name}</span>
                {shown.map(|s| view! {
                    <span class="mt-1 flex flex-wrap items-center gap-x-2 font-mono text-xs">
                        <span class=s.stack.text_class()>{s.stack.label()}</span>
                        <span class="text-muted">"·"</span>
                        <span class="text-muted">{s.proficiency.label()}</span>
                    </span>
                    <span class="mt-4 block h-1 overflow-hidden rounded-full bg-surface-2" aria-hidden="true">
                        <span style:width=format!("{}%", s.level) class=format!("skill-bar block h-full rounded-full {}", s.stack.bar_class()) />
                    </span>
                })}
            }
        >
            <div class="flex flex-col gap-6 sm:flex-row sm:items-start">
                <div class="grid shrink-0 place-items-center self-center rounded-2xl bg-white p-5 ring-1 ring-black/5 sm:self-start">
                    <img src=image_url alt="" width="112" height="112" loading="lazy" decoding="async" class="h-20 w-20 object-contain sm:h-28 sm:w-28" />
                </div>
                <div class="min-w-0 flex-1 space-y-5">
                    {stacks.iter().map(|s| view! {
                        <div>
                            <div class="mb-2 flex flex-wrap items-center justify-between gap-2">
                                <span class=format!("rounded-full border px-3 py-0.5 font-mono text-xs {} {} {}", s.stack.bg_class(), s.stack.text_class(), s.stack.border_class())>
                                    {s.stack.label()}
                                </span>
                                <span class="font-mono text-xs text-muted">{s.proficiency.label()}" · "{s.level}"%"</span>
                            </div>
                            <div class="h-2 overflow-hidden rounded-full bg-surface-2" aria-hidden="true">
                                <div style:width=format!("{}%", s.level) class=format!("skill-bar h-full rounded-full {}", s.stack.bar_class()) />
                            </div>
                        </div>
                    }).collect_view()}
                </div>
            </div>
            <RelatedWorks lang=lang />
            <p class="mt-6 border-t border-line pt-4 text-xs text-muted">{credit}</p>
        </Modal>
    }
}

/// このスキルの言語を使った作品の一覧（なければ何も表示しない）。
#[component]
fn RelatedWorks(lang: Option<Lang>) -> impl IntoView {
    let related: Vec<&'static _> = lang
        .into_iter()
        .flat_map(|lang| {
            WORKS
                .iter()
                .filter(move |work| work.languages.contains(&lang))
        })
        .collect();

    (!related.is_empty()).then(|| {
        view! {
            <section class="mt-8">
                <h3 class="mb-3 font-mono text-xs uppercase tracking-widest text-muted">"Related works"</h3>
                <ul class="grid gap-3 sm:grid-cols-2">
                    {related.into_iter().map(|work| view! {
                        <li><WorkThumb work=work /></li>
                    }).collect_view()}
                </ul>
            </section>
        }
    })
}
