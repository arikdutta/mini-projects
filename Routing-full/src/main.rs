use leptos::prelude::*;

use w4_d18_routing_full::app::App;

// 💁 Run `trunk serve --open` to launch the app 🚀

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App /> }
    })
}
