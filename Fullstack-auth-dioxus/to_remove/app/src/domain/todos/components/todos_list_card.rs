use leptos::prelude::*;

use crate::components::ui::card::{Card, CardContent, CardDescription};
use crate::domain::todos::components::todos_delete_button::TodosDeleteButton;
use crate::domain::todos::todos_services::{DeleteTodo, TodoDto};

#[component]
pub fn TodosListCard(todo: TodoDto, delete_todo: ServerAction<DeleteTodo>) -> impl IntoView {
    view! {
        <Card class="flex gap-2 w-fit">
            <CardContent class="flex flex-col gap-2">
                <CardDescription class="p-2 rounded-md border border-neutral-300">{todo.name.clone()}</CardDescription>
                <TodosDeleteButton unid=todo.unid.to_string() delete_todo=delete_todo />
            </CardContent>
        </Card>
    }
}
