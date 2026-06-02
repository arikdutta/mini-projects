use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {Form, form, "flex flex-col gap-2"}
    clx! {FormField, div, "flex items-center gap-2"}
}

pub use components::*;
