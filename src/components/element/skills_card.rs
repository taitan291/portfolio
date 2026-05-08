use leptos::prelude::*;

use super::modal::Modal;
use crate::data::types::{ChangeLabel, Proficiency, Stack};

#[component]
pub fn SkillsCard(
    #[prop(into)] name: String,
    #[prop(into)] image_url: String,
    skills: Vec<(Stack, Proficiency, u8)>,
    #[prop(into)] credit: String,
) -> impl IntoView {
    let dominant = Stack::dominant(&skills);
    let trigger_name = name.clone();
    let trigger_image = image_url.clone();

    view! {
        <Modal
            title=name
            trigger=move || view! {
                <span class=format!("mx-auto flex h-24 w-24 items-center justify-center rounded-xl {}", dominant.bg_class())>
                    <img src=trigger_image.clone() alt="" class="h-20 w-20 object-contain" />
                </span>
                <span class="mt-3 block break-words text-xl font-medium">{trigger_name.clone()}</span>
            }
        >
            <div class="flex flex-col gap-5 sm:flex-row sm:items-start">
                <div class=format!("self-center shrink-0 rounded-xl p-2 sm:self-start {}", dominant.bg_class())>
                    <img src=image_url alt="" class="h-24 w-24 sm:h-40 sm:w-40 object-contain" />
                </div>
                <div class="min-w-0 flex-1 space-y-4">
                    {skills.into_iter().map(|(stack, proficiency, level)| {
                        let label = proficiency.label();
                        view! {
                            <div>
                                <div class="mb-2 flex flex-wrap items-center justify-between gap-2">
                                    <span class=format!("rounded border px-2 py-1 text-sm {} {} {}", stack.bg_class(), stack.text_class(), stack.border_class())>
                                        {stack.label()}
                                    </span>
                                    <span class=format!("text-sm {}", stack.text_class())>{label}</span>
                                </div>
                                <div class="h-2 overflow-hidden rounded-full bg-cyber-container" aria-hidden="true">
                                    <div style:width=format!("{level}%") class=format!("h-full rounded-full {}", stack.bar_class()) />
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
            <p class="mt-6 border-t border-cyber-border pt-3 text-xs text-cyber-muted">{credit}</p>
        </Modal>
    }
}
