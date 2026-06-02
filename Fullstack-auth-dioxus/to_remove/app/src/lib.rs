#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        // clippy::expect_used,
        // clippy::indexing_slicing,
        // irrefutable_let_patterns,
        clippy::todo,
        // clippy::panic
    )
)]

pub mod common;
pub mod components;
pub mod domain;
pub mod shared;
pub mod utils;

pub mod app;
pub mod shell;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);
}
