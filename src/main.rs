pub mod app;
pub mod components;
pub mod data;
pub mod layouts;
pub mod pages;

#[cfg(target_arch = "wasm32")]
use crate::app::App;

#[cfg(target_arch = "wasm32")]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("This app must be compiled to WebAssembly. Use `trunk serve`.");
}
