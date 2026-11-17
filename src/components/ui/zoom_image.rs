use leptos::html::Dialog;
use leptos::prelude::*;

/// クリックするとネイティブの `<dialog>` で拡大表示するサムネイル。
#[component]
pub fn ZoomImage(
    #[prop(into)] src: String,
    #[prop(into)] alt: String,
    /// サムネイルのボタンに足すクラス（固定サイズの枠を埋める `h-full` など）。
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let dialog = NodeRef::<Dialog>::new();
    let label = format!("{alt} を拡大");
    let dialog_label = alt.clone();

    view! {
        <button
            type="button"
            aria-haspopup="dialog"
            aria-label=label
            class=format!("block w-full cursor-zoom-in {class}")
            on:click=move |_| {
                if let Some(dialog) = dialog.get() {
                    let _ = dialog.show_modal();
                }
            }
        >
            <img src=src.clone() alt=alt.clone() class="h-full w-full rounded-lg object-contain" />
        </button>
        <dialog
            node_ref=dialog
            aria-label=dialog_label
            class="zoom-dialog m-auto overflow-visible bg-transparent p-0 backdrop:bg-black/70"
            on:click=move |_| {
                if let Some(dialog) = dialog.get() {
                    dialog.close();
                }
            }
        >
            <img src=src alt=alt class="zoom-dialog-image cursor-zoom-out rounded-lg object-contain" />
        </dialog>
    }
}
