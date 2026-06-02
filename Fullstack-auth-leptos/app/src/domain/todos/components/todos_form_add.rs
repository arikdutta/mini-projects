use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use web_sys::FormData;

use crate::components::toast_custom::toast_wrapper::show_toast;
use crate::components::ui::button::Button;
use crate::components::ui::input::Input;
use crate::domain::todos::todos_services::AddTodo;

#[component]
pub fn TodosFormAdd(add_todo: ServerMultiAction<AddTodo>) -> impl IntoView {
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let form = ev
            .target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlFormElement>().ok());

        if let Some(form) = form {
            let form_data = FormData::new_with_form(&form).expect("Failed to create FormData");

            let name = form_data.get("name").as_string();

            if let Some(name) = name {
                show_toast().success(format!("Added todo: {name}"))
            }
        }
    };

    view! {
        <MultiActionForm action=add_todo on:submit=on_submit>
            <div class="flex gap-2 items-center p-4 m-4 max-w-md rounded-md border">
                <div>
                    <p>"Name"</p>
                    <Input attr:r#type="text" attr:name="name" />
                </div>

                <Button attr:r#type="submit">"Add"</Button>
            </div>
        </MultiActionForm>
    }
}
