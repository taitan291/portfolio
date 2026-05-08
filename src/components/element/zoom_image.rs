use leptos::{html::Dialog, prelude::*};

/// Thumbnail that opens a larger view of the image in a native modal dialog on click.
#[component]
pub fn ZoomImage(#[prop(into)] src: String, #[prop(into)] alt: String) -> impl IntoView {
    let dialog = NodeRef::<Dialog>::new();
    let label = format!("{alt} を拡大");
    let dialog_label = alt.clone();

    view! {
        <button
            type="button"
            aria-haspopup="dialog"
            aria-label=label
            class="block w-full cursor-zoom-in"
            on:click=move |_| {
                if let Some(dialog) = dialog.get() {
                    let _ = dialog.show_modal();
                }
            }
        >
            <img src=src.clone() alt=alt.clone() class="w-full rounded-lg object-contain" />
        </button>
        <dialog
            node_ref=dialog
            aria-label=dialog_label
            class="m-auto max-h-[95vh] max-w-[95vw] overflow-visible bg-transparent p-0 backdrop:bg-black/70"
            on:click=move |_| {
                if let Some(dialog) = dialog.get() {
                    dialog.close();
                }
            }
        >
            <img src=src alt=alt class="max-h-[95vh] max-w-[95vw] cursor-zoom-out rounded-lg object-contain" />
        </dialog>
    }
}
