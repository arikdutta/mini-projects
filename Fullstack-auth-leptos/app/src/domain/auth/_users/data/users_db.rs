pub struct UsersDb;

#[cfg(feature = "ssr")]
mod db {
    use std::collections::HashSet;

    use serde::{Deserialize, Serialize};
    use sqlx::{PgPool, query_as};
    use time::OffsetDateTime;
    use uuid::Uuid;

    use super::*;
    use crate::domain::auth::_users::data::user::User;
    use crate::domain::auth::_users::pk::UserPk;
    use crate::domain::auth::_users::role::{Role, RoleAccess};
    use crate::utils::types::unid::Unid;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserLoginDto {
        pub unid: Uuid,
        pub email: String,
        pub password: String,
    }

    #[derive(Debug, sqlx::FromRow, Clone)]
    struct SqlUserBasic {
        unid: Unid<UserPk>,
        firstname: String,
        lastname: String,
        email: String,
        password: String,
    }

    #[derive(Debug, sqlx::FromRow, Clone)]
    struct SqlRoleAccess {
        unid: Uuid,
        role: Role,
        grantedto_unid: Uuid,
    }

    impl UsersDb {
        pub async fn get_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
            let sql_users = query_as!(
                SqlUserBasic,
                r#"
                    SELECT 
                        unid, 
                        firstname, 
                        lastname, 
                        email, 
                        password
                    FROM app_schema.users
                    ORDER BY firstname, lastname
                "#
            )
            .fetch_all(pool)
            .await?;

            let mut users = Vec::new();

            for sql_user in sql_users {
                let user_unid = UserPk::from(sql_user.unid.to_uuid());

                // Query role accesses for this user
                let sql_role_accesses = query_as!(
                    SqlRoleAccess,
                    r#"
                        SELECT 
                            unid, 
                            role as "role: Role", 
                            grantedto_unid
                        FROM app_schema.roleaccesses 
                        WHERE grantedto_unid = $1
                    "#,
                    sql_user.unid.as_ref()
                )
                .fetch_all(pool)
                .await?;

                // Convert SqlRoleAccess to RoleAccess
                let roles: HashSet<RoleAccess> = sql_role_accesses
                    .into_iter()
                    .map(|sql_role| {
                        // Use the fields to avoid dead_code warnings
                        tracing::debug!(
                            "Role access {} for user {}",
                            sql_role.unid,
                            sql_role.grantedto_unid
                        );
                        RoleAccess {
                            role: sql_role.role,
                        }
                    })
                    .collect();

                // Build User from SqlUserBasic
                users.push(User {
                    unid: user_unid,
                    created: OffsetDateTime::UNIX_EPOCH,
                    first_name: sql_user.firstname,
                    last_name: sql_user.lastname,
                    last_password_change: OffsetDateTime::UNIX_EPOCH,
                    email: sql_user.email,
                    password: sql_user.password,
                    roles,
                });
            }

            Ok(users)
        }

        pub async fn get_by_unid(
            pool: &PgPool,
            unid: Unid<UserPk>,
        ) -> Result<Option<User>, sqlx::Error> {
            let sql_user = query_as!(
                SqlUserBasic,
                r#"
                    SELECT 
                        unid, 
                        firstname, 
                        lastname, 
                        email, 
                        password
                    FROM app_schema.users
                    WHERE unid = $1
                "#,
                unid.as_ref()
            )
            .fetch_optional(pool)
            .await?;

            if let Some(sql_user) = sql_user {
                // Query role accesses separately
                let sql_role_accesses = query_as!(
                    SqlRoleAccess,
                    r#"
                        SELECT 
                            unid, 
                            role as "role: Role", 
                            grantedto_unid
                        FROM app_schema.roleaccesses 
                        WHERE grantedto_unid = $1
                    "#,
                    unid.as_ref()
                )
                .fetch_all(pool)
                .await?;

                // Convert SqlRoleAccess to RoleAccess
                let roles: HashSet<RoleAccess> = sql_role_accesses
                    .into_iter()
                    .map(|sql_role| {
                        // Use the fields to avoid dead_code warnings
                        tracing::debug!(
                            "Role access {} for user {}",
                            sql_role.unid,
                            sql_role.grantedto_unid
                        );
                        RoleAccess {
                            role: sql_role.role,
                        }
                    })
                    .collect();

                Ok(Some(User {
                    unid: UserPk::from(sql_user.unid.to_uuid()),
                    created: OffsetDateTime::UNIX_EPOCH,
                    first_name: sql_user.firstname,
                    last_name: sql_user.lastname,
                    last_password_change: OffsetDateTime::UNIX_EPOCH,
                    email: sql_user.email,
                    password: sql_user.password,
                    roles,
                }))
            } else {
                Ok(None)
            }
        }

        pub async fn get_from_email(email: String, pool: &PgPool) -> Option<UserLoginDto> {
            query_as!(
                UserLoginDto,
                r#"
                    SELECT 
                        unid,
                        email, 
                        password 
                    FROM app_schema.users
                    WHERE email = $1
                "#,
                email
            )
            .fetch_one(pool)
            .await
            .ok()
        }
    }
}
