use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use uuid::Uuid;

use crate::components::toast_custom::toast_wrapper::show_toast;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::domain::todos::todos_services::DeleteTodo;

#[component]
pub fn TodosDeleteButton(
    #[prop(into)] unid: String,
    delete_todo: ServerAction<DeleteTodo>,
) -> impl IntoView {
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        match unid.parse::<Uuid>() {
            Ok(unid_uuid) => {
                delete_todo.dispatch(DeleteTodo {
                    unid: unid_uuid.into(),
                });
                show_toast().success(format!("🗑️ Deleted todo with unid: {unid_uuid}"));
            }
            Err(_) => show_toast().error("Invalid unid format"),
        }
    };

    /*
       * <ActionForm action=delete_todo on:submit=on_submit>

       * 💁 This creates 2 submissions, which causes a 500 error.
       * So I'm using a regular form for the moment to display the Toast.
       TODO. Find a way to use the ActionForm without the 500 error.
    */

    view! {
        <form on:submit=on_submit>
            <Button attr:r#type="submit" variant=ButtonVariant::Destructive>
                <span>"X"</span>
            </Button>
        </form>
    }
}
