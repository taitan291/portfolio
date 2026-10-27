use leptos::prelude::*;

use super::modal::ModalHandle;
use crate::data::works::Work;

#[component]
pub fn WorkThumb(work: Work) -> impl IntoView {
    let modal = use_context::<ModalHandle>();

    view! {
        <a
            href=format!("#{}", work.id)
            class="relative block h-24 w-24 overflow-hidden rounded-lg"
            on:click=move |_| {
                if let Some(modal) = modal {
                    modal.close();
                }
            }
        >
            <img
                src=work.header_image_url
                alt=""
                loading="lazy"
                class="h-full w-full object-cover"
            />
            <span class="absolute inset-x-0 bottom-0 block break-words bg-black/50 px-3 py-2 text-left text-sm font-medium text-white">
                {work.header_text}
            </span>
        </a>
    }
}
