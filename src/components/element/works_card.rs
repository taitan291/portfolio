use leptos::prelude::*;

use super::modal::Modal;
use super::slideshow::Slideshow;
use crate::data::types::Label;
use crate::data::works::Work;

/// 作品のカード。クリックで説明文と画像をモーダル表示する。
#[component]
pub fn WorkCard(work: &'static Work, index: usize) -> impl IntoView {
    let Work {
        id,
        header_image_url,
        header_text,
        images,
        languages,
        description_html,
        is_public,
        links,
    } = *work;

    view! {
        <Modal
            id=id
            title=header_text
            links=links
            class="overflow-hidden"
            trigger=move || view! {
                <span class="relative block aspect-[16/10] overflow-hidden border-b border-line bg-surface-2">
                    <img
                        src=header_image_url
                        alt=""
                        loading="lazy"
                        decoding="async"
                        class="h-full w-full object-cover transition-transform duration-700 ease-out group-hover:scale-105"
                    />
                    <span class="absolute left-4 top-4 inline-flex items-center gap-1.5 rounded-full bg-black/60 px-3 py-1 font-mono text-xs text-white backdrop-blur">
                        <span class=if is_public { "h-1.5 w-1.5 rounded-full bg-lime-300" } else { "h-1.5 w-1.5 rounded-full bg-neutral-400" } />
                        {if is_public { "Public" } else { "Private" }}
                    </span>
                </span>
                <span class="flex flex-col gap-4 p-5 sm:p-6">
                    <span class="flex items-start justify-between gap-4">
                        <span class="min-w-0">
                            <span class="block font-mono text-xs text-accent-text">{format!("{:02}", index + 1)}</span>
                            <span class="mt-1 block break-words text-2xl font-bold tracking-tight">{header_text}</span>
                        </span>
                        <span
                            class="grid h-11 w-11 shrink-0 place-items-center rounded-full border border-line transition duration-300 group-hover:rotate-45 group-hover:border-accent group-hover:bg-accent group-hover:text-accent-on"
                            aria-hidden="true"
                        >
                            "↗"
                        </span>
                    </span>
                    {(!languages.is_empty()).then(|| view! {
                        <ul aria-label="使用言語" class="flex flex-wrap gap-2">
                            {languages.iter().map(|lang| view! {
                                <li class="tag">{lang.label()}</li>
                            }).collect_view()}
                        </ul>
                    })}
                </span>
            }
        >
            <div class="markdown-content min-w-0 break-words" inner_html=description_html />
            {(!images.is_empty()).then(|| view! { <Slideshow images=images /> })}
        </Modal>
    }
}
