use sqlx::{PgPool, query_as};

use super::pk::TodosPk;
use super::todos_services::TodoDto;
use crate::utils::types::unid::Unid;

pub struct TodosDb {}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

impl TodosDb {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<TodoDto>, sqlx::Error> {
        query_as!(
            TodoDto,
            r#"
                SELECT 
                    unid,
                    name,
                    created_at,
                    updated_at
                FROM app_schema.todos
            "#
        )
        .fetch_all(pool)
        .await
    }

    pub async fn add(pool: &PgPool, name: String) -> Result<TodoDto, sqlx::Error> {
        query_as!(
            TodoDto,
            r#"
                INSERT INTO app_schema.todos (name)
                VALUES ($1)
                RETURNING unid, name, created_at, updated_at
            "#,
            name
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, unid: Unid<TodosPk>) -> Result<Unid<TodosPk>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                DELETE FROM app_schema.todos 
                WHERE unid = $1
                RETURNING unid
            "#,
            unid.as_ref()
        )
        .fetch_one(pool)
        .await?;

        Ok(result.unid.into())
    }
}
