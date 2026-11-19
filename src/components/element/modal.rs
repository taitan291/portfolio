use leptos::{html::Dialog, prelude::*};

use crate::data::types::Link;

/// `Modal` の中身から、そのダイアログを閉じるためのハンドル（ページ内リンクへ移動する前など）。
#[derive(Clone, Copy)]
pub struct ModalHandle(NodeRef<Dialog>);

impl ModalHandle {
    pub fn close(&self) {
        if let Some(dialog) = self.0.get() {
            dialog.close();
        }
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
    /// トリガーのボタンに付ける ID（ページ内リンクの移動先にする）。
    #[prop(optional, into)]
    id: String,
    /// 題名の下に `題名:URL` の形で表示するリンク。URL のスキームは表示しない。
    #[prop(optional)]
    links: &'static [Link],
    /// トリガーのボタンに足すクラス（カードのレイアウトと見た目）。
    #[prop(optional, into)]
    class: String,
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
            class=format!("group card-interactive block h-full w-full min-w-0 text-left {class}")
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
            class="responsive-dialog max-w-3xl overflow-y-auto rounded-3xl border border-line bg-surface p-0 text-fg shadow-2xl backdrop:bg-black/50 backdrop:backdrop-blur-sm"
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
            <div class="sticky top-0 z-10 flex items-center justify-between gap-4 border-b border-line bg-surface/90 px-5 py-4 backdrop-blur sm:px-7">
                <div class="min-w-0">
                    <h2 class="break-words text-2xl font-bold tracking-tight">{title}</h2>
                    {(!links.is_empty()).then(|| view! {
                        <ul class="mt-1 flex flex-col gap-0.5">
                            {links.iter().map(|link| view! {
                                <li class="flex min-w-0 flex-wrap items-baseline gap-x-2 font-mono text-xs">
                                    <span class="text-muted">{link.label}":"</span>
                                    <a
                                        href=link.url
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="inline-flex max-w-full items-center gap-1 break-all text-accent-text underline-offset-4 hover:underline"
                                    >
                                        {display_url(link.url)}
                                        <span aria-hidden="true">"↗"</span>
                                    </a>
                                </li>
                            }).collect_view()}
                        </ul>
                    })}
                </div>
                <button
                    type="button"
                    autofocus
                    aria-label="閉じる"
                    class="grid min-h-11 min-w-11 shrink-0 place-items-center rounded-full border border-line text-lg text-muted transition-colors hover:border-fg/40 hover:text-fg"
                    on:click=move |_| {
                        if let Some(dialog) = dialog.get() {
                            dialog.close();
                        }
                    }
                >"✕"</button>
            </div>
            <div class="p-5 sm:p-7">{children()}</div>
        </dialog>
    }
}
