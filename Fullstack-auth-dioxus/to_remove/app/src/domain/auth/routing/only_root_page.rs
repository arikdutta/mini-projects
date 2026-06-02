use std::collections::HashSet;

use app_macros::secured_server;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::common::app_state::use_app_state;
use crate::common::errors::app_error::AppError;
use crate::components::ui::table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow};
#[cfg(feature = "ssr")]
use crate::domain::auth::_users::data::users_db::UsersDb;
use crate::domain::auth::_users::data::user::User;
use crate::domain::auth::_users::permission::Permission;
use crate::domain::auth::_users::permission_user::UserPermission;
use crate::domain::auth::_users::role::{Role, RoleAccess};
use crate::domain::auth::access_controller::AccessController;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OnlyRootPermission {
    Access,
}

impl Permission for OnlyRootPermission {
    fn roles_required(&self) -> Vec<Role> {
        match self {
            Self::Access => vec![Role::Root],
        }
    }
}


/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                         */
/* ========================================================== */

#[secured_server]
pub async fn get_root_access_data() -> Result<String, AppError> {
    use_app_state()?
        .db_exec(async |_conn, user| -> Result<String, AppError> {
            tracing::info!("Root access check - User UUID: {:?}", user.unid);
            tracing::info!("Root access check - User roles count: {}", user.roles.len());
            tracing::info!("Root access check - User roles: {:?}", user.roles);

            Ok("You have root access!".to_string())
        })
        .await
}

impl AccessController for GetRootAccessData {
    type StateParam = ();

    fn check_permission(user: &User) -> Result<(), AppError> {
        user.check_permission(&OnlyRootPermission::Access)
    }
}

#[secured_server]
pub async fn get_current_user_roles() -> Result<HashSet<RoleAccess>, AppError> {
    use_app_state()?
        .db_exec(
            async |_conn, user| -> Result<HashSet<RoleAccess>, AppError> {
                tracing::info!("User UUID: {:?}", user.unid);
                tracing::info!("User roles count: {}", user.roles.len());
                tracing::info!("User roles: {:?}", user.roles);

                Ok(user.roles.clone())
            },
        )
        .await
}

impl AccessController for GetCurrentUserRoles {
    type StateParam = ();

    fn check_permission(_user: &User) -> Result<(), AppError> {
        // Allow any authenticated user to see their own roles
        Ok(())
    }
}

#[secured_server]
pub async fn get_all_users() -> Result<Vec<User>, AppError> {
    let app_state = use_app_state()?;

    let users = UsersDb::get_all(&app_state.pool)
        .await
        .map_err(AppError::from)?;

    Ok(users)
}

impl AccessController for GetAllUsers {
    type StateParam = ();

    fn check_permission(user: &User) -> Result<(), AppError> {
        user.check_permission(&UserPermission::ListAll)
    }
}

#[component]
pub fn OnlyRootPage() -> impl IntoView {
    let access_data =
        leptos::prelude::Resource::new(|| (), |_| async { get_root_access_data().await });

    let user_roles =
        leptos::prelude::Resource::new(|| (), |_| async { get_current_user_roles().await });

    let all_users = leptos::prelude::Resource::new(|| (), |_| async { get_all_users().await });

    view! {
        <div class="p-6">
            <h1 class="mb-6 text-3xl font-bold">"Only Root Page"</h1>

            <div class="mb-8">
                <h2 class="mb-4 text-xl font-semibold">"Root Access Check"</h2>
                <Suspense fallback=move || {
                    view! { <p class="text-gray-500">"Loading access check..."</p> }
                }>
                    {move || {
                        access_data
                            .get()
                            .map(|result| {
                                match result {
                                    Ok(message) => {
                                        view! {
                                            <div class="py-3 px-4 text-green-700 bg-green-100 rounded border border-green-400">
                                                <p class="font-medium">{message}</p>
                                            </div>
                                        }
                                    }
                                    Err(err) => {
                                        view! {
                                            <div class="py-3 px-4 text-red-700 bg-red-100 rounded border border-red-400">
                                                <p class="font-medium">"Access denied: " {err.to_string()}</p>
                                            </div>
                                        }
                                    }
                                }
                            })
                    }}
                </Suspense>
            </div>

            <div>
                <h2 class="mb-4 text-xl font-semibold">"Your Current Roles"</h2>
                <Suspense fallback=move || {
                    view! { <p class="text-gray-500">"Loading user roles..."</p> }
                }>
                    {move || {
                        user_roles
                            .get()
                            .map(|result| {
                                match result {
                                    Ok(roles) => {
                                        if roles.is_empty() {
                                            view! {
                                                <div class="py-3 px-4 text-yellow-700 bg-yellow-100 rounded border border-yellow-400">
                                                    <p>"No roles assigned"</p>
                                                </div>
                                            }
                                        } else {
                                            view! {
                                                <div class="p-4 bg-blue-50 rounded-lg border border-blue-200">
                                                    <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
                                                        {roles
                                                            .iter()
                                                            .map(|role_access| {
                                                                view! {
                                                                    <div class="p-3 bg-white rounded-lg border border-gray-200 shadow-sm">
                                                                        <div class="font-medium text-gray-900">
                                                                            {format!("{:?}", role_access.role)}
                                                                        </div>
                                                                        <div class="mt-1 text-sm text-gray-500">"Global Role"</div>
                                                                    </div>
                                                                }
                                                            })
                                                            .collect::<Vec<_>>()}
                                                    </div>
                                                    <div class="mt-4 text-sm text-gray-600">
                                                        "Total roles: " {roles.len()}
                                                    </div>
                                                </div>
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        view! {
                                            <div class="py-3 px-4 text-red-700 bg-red-100 rounded border border-red-400">
                                                <p class="font-medium">"Error loading roles: " {err.to_string()}</p>
                                            </div>
                                        }
                                    }
                                }
                            })
                    }}
                </Suspense>
            </div>

            <div class="mt-8">
                <h2 class="mb-4 text-xl font-semibold">"All Users"</h2>
                <Suspense fallback=move || {
                    view! { <p class="text-gray-500">"Loading users..."</p> }
                }>
                    {move || {
                        all_users
                            .get()
                            .map(|result| {
                                match result {
                                    Ok(users) => {
                                        view! {
                                            <div>
                                                {if users.is_empty() {
                                                    view! {
                                                        <div class="py-3 px-4 text-yellow-700 bg-yellow-100 rounded border border-yellow-400">
                                                            <p>"No users found"</p>
                                                        </div>
                                                    }
                                                        .into_any()
                                                } else {
                                                    view! {
                                                        <Table>
                                                            <TableHeader>
                                                                <TableRow>
                                                                    <TableHead>{"Name"}</TableHead>
                                                                    <TableHead>{"Email"}</TableHead>
                                                                    <TableHead>{"Roles"}</TableHead>
                                                                </TableRow>
                                                            </TableHeader>
                                                            <TableBody>
                                                                {users
                                                                    .iter()
                                                                    .map(|user| {
                                                                        let user = user.clone();
                                                                        let full_name = user.full_name();
                                                                        let display_name = if full_name.is_empty() {
                                                                            "N/A".to_string()
                                                                        } else {
                                                                            full_name
                                                                        };
                                                                        let roles_display = if user.roles.is_empty() {
                                                                            "No roles".to_string()
                                                                        } else {
                                                                            user.roles
                                                                                .iter()
                                                                                .map(|role| format!("{:?}", role.role))
                                                                                .collect::<Vec<_>>()
                                                                                .join(", ")
                                                                        };

                                                                        view! {
                                                                            <TableRow>
                                                                                <TableCell>{display_name}</TableCell>
                                                                                <TableCell>{user.email}</TableCell>
                                                                                <TableCell>{roles_display}</TableCell>
                                                                            </TableRow>
                                                                        }
                                                                    })
                                                                    .collect::<Vec<_>>()}
                                                            </TableBody>
                                                        </Table>
                                                    }
                                                        .into_any()
                                                }}
                                            </div>
                                        }
                                    }
                                    Err(err) => {
                                        view! {
                                            <div>
                                                <div class="py-3 px-4 text-red-700 bg-red-100 rounded border border-red-400">
                                                    <p class="font-medium">"Error loading users: " {err.to_string()}</p>
                                                </div>
                                            </div>
                                        }
                                    }
                                }
                            })
                    }}
                </Suspense>
            </div>
        </div>
    }
}
