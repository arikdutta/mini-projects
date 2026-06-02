use leptos::either::Either;
use leptos::prelude::*;

use crate::common::error_template::ErrorTemplate;
use crate::domain::todos::components::todos_form_add::TodosFormAdd;
use crate::domain::todos::components::todos_list_card::TodosListCard;
use crate::domain::todos::todos_services::{get_todos, AddTodo, DeleteTodo};

#[component]
pub fn TodosList() -> impl IntoView {
    let add_todo = ServerMultiAction::<AddTodo>::new();
    let delete_todo = ServerAction::<DeleteTodo>::new();

    let todos = Resource::new(
        move || (add_todo.version().get(), delete_todo.version().get()),
        move |_| get_todos(),
    );

    view! {
        <TodosFormAdd add_todo=add_todo />

        <Transition fallback=move || view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorTemplate errors /> }
            }>
                {move || {
                    todos
                        .and_then(|todos| {
                            if todos.is_empty() {
                                Either::Left(view! { <p>"No tasks were found."</p> })
                            } else {
                                Either::Right(
                                    todos
                                        .iter()
                                        .map(|todo| {
                                            view! { <TodosListCard todo=todo.clone() delete_todo=delete_todo /> }
                                        })
                                        .collect::<Vec<_>>(),
                                )
                            }
                        })
                }}
            </ErrorBoundary>
        </Transition>
    }
}
