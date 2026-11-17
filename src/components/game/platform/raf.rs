//! `requestAnimationFrame` ループ。`stop()` で次のフレームを取り消し、クロージャを解放する。

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

type FrameFn = Closure<dyn FnMut(f64)>;

struct Inner {
    handle: Cell<Option<i32>>,
    closure: RefCell<Option<FrameFn>>,
}

impl Inner {
    fn request(&self) {
        let Some(window) = web_sys::window() else {
            return;
        };
        if let Some(closure) = self.closure.borrow().as_ref() {
            let handle = window
                .request_animation_frame(closure.as_ref().unchecked_ref())
                .ok();
            self.handle.set(handle);
        }
    }
}

pub struct RafLoop {
    inner: Rc<Inner>,
}

impl RafLoop {
    /// 毎フレーム `frame(timestamp_ms)` を呼ぶループを開始する。
    pub fn start(mut frame: impl FnMut(f64) + 'static) -> Self {
        let inner = Rc::new(Inner {
            handle: Cell::new(None),
            closure: RefCell::new(None),
        });
        // クロージャからは弱参照で辿り、循環参照を作らない。
        let weak = Rc::downgrade(&inner);
        let closure = Closure::<dyn FnMut(f64)>::new(move |ts: f64| {
            frame(ts);
            if let Some(inner) = weak.upgrade() {
                inner.request();
            }
        });
        *inner.closure.borrow_mut() = Some(closure);
        inner.request();
        Self { inner }
    }

    pub fn stop(&self) {
        if let (Some(handle), Some(window)) = (self.inner.handle.take(), web_sys::window()) {
            let _ = window.cancel_animation_frame(handle);
        }
        self.inner.closure.borrow_mut().take();
    }
}
