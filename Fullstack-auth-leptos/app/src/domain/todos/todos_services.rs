use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::domain::todos::pk::TodosPk;
use crate::utils::types::unid::Unid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct TodoDto {
    pub unid: Unid<TodosPk>,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[server(GetTodos)]
pub async fn get_todos() -> Result<Vec<TodoDto>, ServerFnError> {
    use crate::common::app_state::use_app_state;
    use crate::domain::todos::todos_db::TodosDb;

    let app_state = use_app_state()?;

    let todos = TodosDb::get_all(&app_state.pool)
        .await
        .map_err(ServerFnError::new)?;

    Ok(todos)
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[server(AddTodo)]
pub async fn add_todo(name: String) -> Result<TodoDto, ServerFnError> {
    use crate::common::app_state::use_app_state;
    use crate::domain::todos::todos_db::TodosDb;

    let app_state = use_app_state()?;

    let todo = TodosDb::add(&app_state.pool, name)
        .await
        .map_err(ServerFnError::new)?;

    Ok(todo)
}

#[server(DeleteTodo)]
pub async fn delete_todo(unid: Unid<TodosPk>) -> Result<Unid<TodosPk>, ServerFnError> {
    use crate::common::app_state::use_app_state;
    use crate::domain::todos::todos_db::TodosDb;

    let app_state = use_app_state()?;

    let todo_unid = TodosDb::delete(&app_state.pool, unid)
        .await
        .map_err(ServerFnError::new)?;

    Ok(todo_unid)
}
