use leptos::prelude::*;

use crate::domain::auth::_users::service::get_user::get_user;
use crate::domain::auth::_users::service::login::Login;
use crate::domain::auth::_users::service::logout::Logout;

#[component]
pub fn ProtectedPage() -> impl IntoView {
    let login = ServerAction::<Login>::new();
    let logout = ServerAction::<Logout>::new();

    let user_resource = Resource::new(
        move || (login.version().get(), logout.version().get()),
        move |_| get_user(),
    );

    view! {
        <div class="container p-4 mx-auto">
            <h1 class="mb-4 text-2xl font-bold">"Protected Page"</h1>

            <Suspense fallback=move || {
                view! { <span>"Checking authentication status..."</span> }
            }>
                {move || {
                    user_resource
                        .get()
                        .map(|user| match user {
                            Ok(Some(user)) => {
                                view! {
                                    <div class="p-4 bg-green-50 rounded-md border">
                                        <h2 class="mb-2 text-xl font-semibold">
                                            "Welcome, " {user.first_name.clone()} " " {user.last_name.clone()}
                                        </h2>
                                        <p>"You have access to this protected content."</p>
                                    </div>
                                }
                                    .into_any()
                            }
                            Ok(None) => {
                                view! {
                                    <div class="p-4 bg-red-50 rounded-md border">
                                        <h2 class="mb-2 text-xl font-semibold">"Authentication Required"</h2>
                                        <p>"You must be logged in to access this page."</p>
                                    </div>
                                }
                                    .into_any()
                            }
                            Err(err) => {
                                view! {
                                    <div class="p-4 bg-yellow-50 rounded-md border">
                                        <p>"Error checking authentication: " {format!("{err:?}")}</p>
                                    </div>
                                }
                                    .into_any()
                            }
                        })
                        .or_else(|| { Some(view! { "" }.into_any()) })
                }}
            </Suspense>
        </div>
    }
}
