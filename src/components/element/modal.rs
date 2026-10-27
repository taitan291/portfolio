use leptos::{html::Dialog, prelude::*};

/// Lets content inside a `Modal` close its dialog (e.g. before jumping to an in-page anchor).
#[derive(Clone, Copy)]
pub struct ModalHandle(NodeRef<Dialog>);

impl ModalHandle {
    pub fn close(&self) {
        if let Some(dialog) = self.0.get() {
            dialog.close();
        }
    }
}

/// Native modal dialogs provide Escape handling, focus containment and focus restoration.
#[component]
pub fn Modal(
    #[prop(into)] title: String,
    #[prop(into)] trigger: ViewFn,
    /// Anchor id on the trigger button, so in-page links can jump to it.
    #[prop(optional, into)]
    id: String,
    /// Full URL shown under the title in the header; the scheme is stripped when displayed.
    #[prop(optional)]
    url: &'static str,
    children: Children,
) -> impl IntoView {
    let dialog = NodeRef::<Dialog>::new();
    let accessible_title = title.clone();
    provide_context(ModalHandle(dialog));

    view! {
        <button
            type="button"
            aria-haspopup="dialog"
            id=(!id.is_empty()).then_some(id)
            class="scroll-mt-6 min-w-0 w-full rounded-xl border border-cyber-border bg-white p-4 sm:p-5 text-center transition-colors hover:border-neutral-400 hover:bg-cyber-container"
            on:click=move |_| {
                if let Some(dialog) = dialog.get() {
                    let _ = dialog.show_modal();
                }
            }
        >
            {trigger.run()}
        </button>
        <dialog
            node_ref=dialog
            aria-label=accessible_title
            class="responsive-dialog max-w-3xl overflow-y-auto rounded-2xl bg-white p-0 text-cyber-text shadow-xl backdrop:bg-black/20 backdrop:backdrop-blur-sm"
            on:click=move |event| {
                if let Some(dialog) = dialog.get() {
                    let bounds = dialog.get_bounding_client_rect();
                    let x = f64::from(event.client_x());
                    let y = f64::from(event.client_y());
                    if event.target() == event.current_target()
                        && (x < bounds.left() || x > bounds.right()
                            || y < bounds.top() || y > bounds.bottom())
                    {
                        dialog.close();
                    }
                }
            }
        >
            <div class="sticky top-0 z-10 flex items-center justify-between gap-4 border-b border-cyber-border bg-white p-4">
                <div class="min-w-0">
                    <h2 class="break-words text-2xl font-bold text-cyber-accent">{title}</h2>
                    {(!url.is_empty()).then(|| {
                        let display = url
                            .strip_prefix("https://")
                            .or_else(|| url.strip_prefix("http://"))
                            .unwrap_or(url);
                        view! {
                            <a
                                href=url
                                target="_blank"
                                rel="noopener noreferrer"
                                class="mt-1 block break-all text-sm text-blue-600 underline"
                            >
                                {display}
                            </a>
                        }
                    })}
                </div>
                <button
                    type="button"
                    autofocus
                    aria-label="閉じる"
                    class="min-h-11 min-w-11 shrink-0 rounded px-3 py-2 text-xl transition-colors hover:bg-cyber-container"
                    on:click=move |_| {
                        if let Some(dialog) = dialog.get() {
                            dialog.close();
                        }
                    }
                >"✕"</button>
            </div>
            <div class="p-4 sm:p-5">{children()}</div>
        </dialog>
    }
}
