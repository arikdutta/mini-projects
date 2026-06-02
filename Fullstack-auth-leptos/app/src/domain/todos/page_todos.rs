use leptos::prelude::*;

use crate::domain::todos::components::todos_list::TodosList;

#[component]
pub fn PageTodos() -> impl IntoView {
    view! {
        <h1>"Todos"</h1>

        <TodosList />
    }
}
