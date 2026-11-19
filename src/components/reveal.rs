//! Scroll-triggered reveal animations.
//!
//! Elements marked with `data-reveal` start hidden (see `style.css`) and get the
//! `is-revealed` class the first time they scroll into view. The hidden state is
//! only enabled once the observer is running, so content never stays invisible
//! when IntersectionObserver is unavailable.

use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

/// Class on `<html>` that turns on the "before reveal" styles.
const ENABLED_CLASS: &str = "js-reveal";
const REVEALED_CLASS: &str = "is-revealed";
const TARGET_SELECTOR: &str = "[data-reveal]";

type ObserverCallback = dyn FnMut(js_sys::Array, IntersectionObserver);

/// Builds the inline style that staggers a reveal by `step` slots of 70ms.
pub fn reveal_delay(step: usize) -> String {
    format!("--reveal-delay: {}ms", step * 70)
}

/// Observes every `[data-reveal]` element inside `root` once it is mounted.
pub fn use_reveal(root: NodeRef<html::Div>) {
    Effect::new(move |_| root.get().and_then(|root| RevealObserver::new(&root)));
}

struct RevealObserver {
    observer: IntersectionObserver,
    // Must outlive the observer, which calls into it.
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
        // Trigger slightly before the element's top reaches the bottom edge.
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
