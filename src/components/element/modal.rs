use leptos::{
    html::{Button, Dialog},
    prelude::*,
};
use web_sys::HtmlDialogElement;

/// Feeds the clicked card's rect to the CSS reveal animation as `--from-*` insets
/// (distance from each edge of the dialog to the matching edge of the card).
fn set_origin(dialog: &HtmlDialogElement, card: &web_sys::Element) {
    let d = dialog.get_bounding_client_rect();
    let c = card.get_bounding_client_rect();
    let style = web_sys::HtmlElement::style(dialog);
    for (name, value) in [
        ("--from-top", c.top() - d.top()),
        ("--from-right", d.right() - c.right()),
        ("--from-bottom", d.bottom() - c.bottom()),
        ("--from-left", c.left() - d.left()),
    ] {
        let _ = style.set_property(name, &format!("{value}px"));
    }
}

/// Native modal dialogs provide Escape handling, focus containment and focus restoration.
#[component]
pub fn Modal(
    #[prop(into)] title: String,
    #[prop(into)] trigger: ViewFn,
    /// `(題名, url)` pairs shown under the title as `題名:url`; the scheme is stripped when displayed.
    #[prop(optional)]
    urls: &'static [(&'static str, &'static str)],
    children: Children,
) -> impl IntoView {
    let dialog = NodeRef::<Dialog>::new();
    let card = NodeRef::<Button>::new();
    let accessible_title = title.clone();

    // Play the reverse animation, then close on `animationend`.
    let request_close = move || {
        if let (Some(dialog), Some(card)) = (dialog.get(), card.get()) {
            if dialog.get_attribute("data-anim").as_deref() == Some("close") {
                return;
            }
            set_origin(&dialog, &card);
            let _ = dialog.set_attribute("data-anim", "close");
        }
    };

    view! {
        <button
            node_ref=card
            type="button"
            aria-haspopup="dialog"
            class="min-w-0 w-full rounded-xl border border-cyber-border bg-white p-4 sm:p-5 text-center transition-colors hover:border-neutral-400 hover:bg-cyber-container hover:-translate-y-[5px]"
            on:click=move |_| {
                if let (Some(dialog), Some(card)) = (dialog.get(), card.get()) {
                    let _ = dialog.show_modal();
                    // Measure before data-anim is set so the animation can't skew the rect.
                    set_origin(&dialog, &card);
                    let _ = dialog.set_attribute("data-anim", "open");
                }
            }
        >
            {trigger.run()}
        </button>
        <dialog
            node_ref=dialog
            aria-label=accessible_title
            class="responsive-dialog max-w-3xl overflow-y-auto rounded-2xl bg-white p-0 text-cyber-text shadow-xl backdrop:bg-black/20 backdrop:backdrop-blur-sm"
            on:cancel=move |event: web_sys::Event| {
                event.prevent_default();
                request_close();
            }
            on:animationend=move |event| {
                if event.target() != event.current_target() {
                    return;
                }
                if let Some(dialog) = dialog.get() {
                    let closing = dialog.get_attribute("data-anim").as_deref() == Some("close");
                    let _ = dialog.remove_attribute("data-anim");
                    if closing {
                        dialog.close();
                    }
                }
            }
            on:click=move |event| {
                if let Some(dialog) = dialog.get() {
                    let bounds = dialog.get_bounding_client_rect();
                    let x = f64::from(event.client_x());
                    let y = f64::from(event.client_y());
                    if event.target() == event.current_target()
                        && (x < bounds.left() || x > bounds.right()
                            || y < bounds.top() || y > bounds.bottom())
                    {
                        request_close();
                    }
                }
            }
        >
            <div class="sticky top-0 z-10 flex items-center justify-between gap-4 border-b border-cyber-border bg-white p-4">
                <div class="min-w-0">
                    <h2 class="break-words text-2xl font-bold text-cyber-accent">{title}</h2>
                    {urls.iter().map(|&(label, url)| {
                        let display = url
                            .strip_prefix("https://")
                            .or_else(|| url.strip_prefix("http://"))
                            .unwrap_or(url);
                        view! {
                            <p class="mt-1 break-all text-sm">
                                {label}":"
                                <a
                                    href=url
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="text-blue-600 underline"
                                >
                                    {display}
                                </a>
                            </p>
                        }
                    }).collect_view()}
                </div>
                <button
                    type="button"
                    autofocus
                    aria-label="閉じる"
                    class="min-h-11 min-w-11 shrink-0 rounded px-3 py-2 text-xl transition-colors hover:bg-cyber-container"
                    on:click=move |_| request_close()
                >"✕"</button>
            </div>
            <div class="p-4 sm:p-5">{children()}</div>
        </dialog>
    }
}
