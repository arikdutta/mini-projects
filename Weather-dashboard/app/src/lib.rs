#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]

pub mod routing;

pub mod app;
pub mod components;
pub mod shell;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
