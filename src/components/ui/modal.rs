use leptos::html::{Button, Dialog};
use leptos::prelude::*;
use web_sys::{Element, HtmlDialogElement, HtmlElement};

use crate::data::types::Link;

/// クリックしたカードの位置を、CSS の展開アニメーション用に `--from-*` として渡す
/// （ダイアログの各辺から、カードの対応する辺までの距離）。
fn set_origin(dialog: &HtmlDialogElement, card: &Element) {
    let d = dialog.get_bounding_client_rect();
    let c = card.get_bounding_client_rect();
    // `dialog.style()` は Leptos の `ElementExt::style` に隠れるので、明示的に呼ぶ。
    let style = HtmlElement::style(dialog);
    for (name, value) in [
        ("--from-top", c.top() - d.top()),
        ("--from-right", d.right() - c.right()),
        ("--from-bottom", d.bottom() - c.bottom()),
        ("--from-left", c.left() - d.left()),
    ] {
        let _ = style.set_property(name, &format!("{value}px"));
    }
}

/// URL からスキームを除いた表示用の文字列。
fn display_url(url: &str) -> &str {
    url.strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url)
}

/// カード型のボタンと、それで開くモーダル。
/// ネイティブの `<dialog>` を使い、Esc での終了・フォーカスの閉じ込めと復帰はブラウザに任せる。
#[component]
pub fn Modal(
    #[prop(into)] title: String,
    #[prop(into)] trigger: ViewFn,
    /// 題名の下に `題名:URL` の形で表示するリンク。URL のスキームは表示しない。
    #[prop(optional)]
    links: &'static [Link],
    children: Children,
) -> impl IntoView {
    let dialog = NodeRef::<Dialog>::new();
    let card = NodeRef::<Button>::new();
    let accessible_title = title.clone();

    // 逆再生のアニメーションを流し、`animationend` で閉じる。
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
            class="min-w-0 w-full rounded-xl border border-site-border bg-site-surface p-4 sm:p-5 text-center transition-colors hover:border-site-border-strong hover:bg-site-subtle [@media(hover:hover)]:hover:-translate-y-[5px]"
            on:click=move |_| {
                if let (Some(dialog), Some(card)) = (dialog.get(), card.get()) {
                    let _ = dialog.show_modal();
                    // アニメーションで位置がずれないよう、data-anim を付ける前に測る。
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
            class="responsive-dialog max-w-3xl overflow-y-auto rounded-2xl bg-site-surface p-0 text-site-text shadow-xl backdrop:bg-black/20 backdrop:backdrop-blur-sm"
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
            <div class="sticky top-0 z-10 flex items-center justify-between gap-4 border-b border-site-border bg-site-surface p-4">
                <div class="min-w-0">
                    <h2 class="break-words text-2xl font-bold text-site-accent">{title}</h2>
                    {links.iter().map(|link| view! {
                        <p class="mt-1 break-all text-sm">
                            {link.label}":"
                            <a
                                href=link.url
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-site-link underline"
                            >
                                {display_url(link.url)}
                            </a>
                        </p>
                    }).collect_view()}
                </div>
                <button
                    type="button"
                    autofocus
                    aria-label="閉じる"
                    class="min-h-11 min-w-11 shrink-0 rounded px-3 py-2 text-xl transition-colors hover:bg-site-subtle"
                    on:click=move |_| request_close()
                >"✕"</button>
            </div>
            <div class="p-4 sm:p-5">{children()}</div>
        </dialog>
    }
}
