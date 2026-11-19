use leptos::prelude::*;

use super::modal::Modal;
use super::work_thumb::WorkThumb;
use crate::data::types::{ChangeLabel, Lang, Proficiency, Stack};
use crate::data::works::WORKS;

#[component]
pub fn SkillsCard(
    #[prop(into)] name: String,
    #[prop(into)] image_url: String,
    skills: Vec<(Stack, Proficiency, u8)>,
    #[prop(into)] credit: String,
    lang: Option<Lang>,
) -> impl IntoView {
    let trigger_name = name.clone();
    let trigger_image = image_url.clone();
    let trigger_skills = skills.clone();

    view! {
        <Modal
            title=name
            class="flex flex-col p-5"
            trigger=move || {
                let (stack, proficiency, level) = trigger_skills
                    .first()
                    .copied()
                    .unwrap_or((Stack::dominant(&trigger_skills), Proficiency::Beginner, 0));
                view! {
                    <span class="flex items-start justify-between gap-3">
                        <span class="grid h-14 w-14 shrink-0 place-items-center rounded-xl bg-white ring-1 ring-black/5 transition-transform duration-300 group-hover:scale-105">
                            <img src=trigger_image.clone() alt="" class="h-9 w-9 object-contain" />
                        </span>
                        <span class="font-mono text-xs text-muted transition-colors group-hover:text-fg" aria-hidden="true">"↗"</span>
                    </span>
                    <span class="mt-5 block break-words text-xl font-bold tracking-tight">{trigger_name.clone()}</span>
                    <span class="mt-1 flex flex-wrap items-center gap-x-2 font-mono text-xs">
                        <span class=stack.text_class()>{stack.label()}</span>
                        <span class="text-muted">"·"</span>
                        <span class="text-muted">{proficiency.label()}</span>
                    </span>
                    <span class="mt-4 block h-1 overflow-hidden rounded-full bg-surface-2" aria-hidden="true">
                        <span style:width=format!("{level}%") class=format!("block h-full rounded-full {}", stack.bar_class()) />
                    </span>
                }
            }
        >
            <div class="flex flex-col gap-6 sm:flex-row sm:items-start">
                <div class="grid shrink-0 place-items-center self-center rounded-2xl bg-white p-5 ring-1 ring-black/5 sm:self-start">
                    <img src=image_url alt="" class="h-20 w-20 object-contain sm:h-28 sm:w-28" />
                </div>
                <div class="min-w-0 flex-1 space-y-5">
                    {skills.into_iter().map(|(stack, proficiency, level)| view! {
                        <div>
                            <div class="mb-2 flex flex-wrap items-center justify-between gap-2">
                                <span class=format!("rounded-full border px-3 py-0.5 font-mono text-xs {} {} {}", stack.bg_class(), stack.text_class(), stack.border_class())>
                                    {stack.label()}
                                </span>
                                <span class="font-mono text-xs text-muted">{proficiency.label()}" · "{level}"%"</span>
                            </div>
                            <div class="h-2 overflow-hidden rounded-full bg-surface-2" aria-hidden="true">
                                <div style:width=format!("{level}%") class=format!("h-full rounded-full {}", stack.bar_class()) />
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

/// Lists the works using this skill's language (nothing when there are none).
#[component]
fn RelatedWorks(lang: Option<Lang>) -> impl IntoView {
    let related: Vec<_> = lang
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
                        <li><WorkThumb work=*work /></li>
                    }).collect_view()}
                </ul>
            </section>
        }
    })
}
