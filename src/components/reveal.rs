//! スクロールでのフェードイン。
//!
//! `data-reveal` を付けた要素は最初は隠れていて（`style.css` 参照）、初めて画面に入ったときに
//! `is-revealed` クラスが付く。隠す指定は監視が始まってから有効にするので、
//! IntersectionObserver が使えない環境でも内容が見えないままになることはない。

use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

/// 「表示前」のスタイルを有効にする `<html>` のクラス。
const ENABLED_CLASS: &str = "js-reveal";
const REVEALED_CLASS: &str = "is-revealed";
const TARGET_SELECTOR: &str = "[data-reveal]";

type ObserverCallback = dyn FnMut(js_sys::Array, IntersectionObserver);

/// フェードインを `step` × 70ms 遅らせるインラインスタイルを作る。
pub fn reveal_delay(step: usize) -> String {
    format!("--reveal-delay: {}ms", step * 70)
}

/// `root` のマウント後、その中の `[data-reveal]` 要素をすべて監視する。
pub fn use_reveal(root: NodeRef<html::Div>) {
    Effect::new(move |_| root.get().and_then(|root| RevealObserver::new(&root)));
}

struct RevealObserver {
    observer: IntersectionObserver,
    // observer から呼ばれるので、observer より長く生かしておく。
    _callback: Closure<ObserverCallback>,
}

impl RevealObserver {
    fn new(root: &Element) -> Option<Self> {
        let callback = Closure::<ObserverCallback>::new(
            |entries: js_sys::Array, observer: IntersectionObserver| {
                for entry in entries.iter() {
                    let entry: IntersectionObserverEntry = entry.unchecked_into();
                    if entry.is_intersecting() {
                        let target = entry.target();
                        let _ = target.class_list().add_1(REVEALED_CLASS);
                        observer.unobserve(&target);
                    }
                }
            },
        );

        let options = IntersectionObserverInit::new();
        // 要素の上端が画面の下端に届く少し手前で発火させる。
        options.set_root_margin("0px 0px -8% 0px");
        options.set_threshold(&JsValue::from_f64(0.0));
        let observer =
            IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options)
                .ok()?;

        let targets = root.query_selector_all(TARGET_SELECTOR).ok()?;
        for index in 0..targets.length() {
            if let Some(node) = targets.item(index) {
                observer.observe(node.unchecked_ref());
            }
        }

        if let Some(document_element) = root.owner_document().and_then(|d| d.document_element()) {
            let _ = document_element.class_list().add_1(ENABLED_CLASS);
        }

        Some(Self {
            observer,
            _callback: callback,
        })
    }
}

impl Drop for RevealObserver {
    fn drop(&mut self) {
        self.observer.disconnect();
    }
}
