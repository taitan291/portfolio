use leptos::prelude::*;

use crate::components::ui::modal::Modal;
use crate::components::ui::slideshow::Slideshow;
use crate::data::works::Work;

#[component]
pub fn WorkCard(work: Work) -> impl IntoView {
    let Work {
        header_image_url,
        header_text,
        images,
        languages,
        description_html,
        is_public,
        urls,
    } = work;

    view! {
        <Modal
            title=header_text
            urls=urls
            trigger=move || view! {
                <span class="relative block overflow-hidden rounded-lg">
                    <img src=header_image_url alt="" class="h-52 w-full object-cover" />
                    <span class="absolute inset-x-0 bottom-0 block break-words bg-black/50 px-3 py-2 text-left text-lg font-medium text-white">
                        {header_text}
                    </span>
                </span>
                <div class="mt-3 flex items-start justify-between gap-2">
                    <span class="shrink-0 rounded border border-site-muted/40 px-2 py-1 text-sm text-site-muted">
                        {if is_public { "Public" } else { "Private" }}
                    </span>
                    <ul aria-label="使用言語" class="flex flex-wrap justify-end gap-2">
                        {languages.iter().map(|lang| view! {
                            <li class="rounded bg-site-subtle px-2 py-1 text-sm">{lang.label()}</li>
                        }).collect_view()}
                    </ul>
                </div>
            }
        >
            <div class="markdown-content min-w-0 break-words" inner_html=description_html />
            {(!images.is_empty()).then(|| view! { <Slideshow images=images /> })}
        </Modal>
    }
}
