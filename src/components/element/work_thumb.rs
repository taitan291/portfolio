use leptos::prelude::*;

use super::modal::ModalHandle;
use crate::data::works::Work;

/// ページ内の作品カードへの小さなリンク（スキルのダイアログ内で使う）。
#[component]
pub fn WorkThumb(work: &'static Work) -> impl IntoView {
    let modal = use_context::<ModalHandle>();

    view! {
        <a
            href=format!("#{}", work.id)
            class="group flex items-center gap-3 rounded-xl border border-line p-2 pr-4 transition-colors hover:border-fg/30 hover:bg-surface-2"
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
                class="h-14 w-14 shrink-0 rounded-lg object-cover"
            />
            <span class="min-w-0 flex-1 break-words text-sm font-medium">{work.header_text}</span>
            <span class="font-mono text-xs text-muted transition-transform group-hover:translate-x-0.5" aria-hidden="true">"→"</span>
        </a>
    }
}
