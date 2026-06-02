use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::StaticSegment;
use leptos_router::components::{Outlet, ProtectedParentRoute, Route, Router, Routes};
use leptos_router::hooks::use_location;
use leptos_router::path;

use crate::common::error_template::ErrorTemplate;
use crate::common::errors::app_error::AppError;
use crate::components::navigation::header::Header;
use crate::components::toast_custom::toaster::{Toaster, provide_toaster};
use crate::domain::auth::_users::service::get_user::get_user;
use crate::domain::auth::routing::login_page::LoginPage;
use crate::domain::auth::routing::only_root_page::OnlyRootPage;
use crate::domain::auth::routing::protected_page::ProtectedPage;
use crate::domain::todos::page_todos::PageTodos;
use crate::shared::routing::page_test::PageTest;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    provide_toaster();

    // Create user resource for authentication checking
    let user_resource = Resource::new(|| (), |_| get_user());

    // Authentication condition - returns Option<bool>
    let is_authenticated = move || {
        user_resource.get().map(|result| match result {
            Ok(Some(_)) => true, // Authenticated user
            Ok(None) => false,   // No user (unauthenticated)
            Err(_) => false,     // Error treated as unauthenticated
        })
    };

    view! {
        <Router>
            <Toaster />

            <Header />

            <main class="p-2">
                <Routes fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::not_found(use_location().pathname.get()));
                    view! { <ErrorTemplate outside_errors /> }
                }>
                    // ------------ 🌐 PUBLIC ROUTES 🌐 ------------ //
                    <Route path=StaticSegment("/") view=PageHome />
                    <Route path=StaticSegment("/test") view=PageTest />
                    <Route path=StaticSegment("/login") view=LoginPage />

                    // ------------ 🔒 PROTECTED ROUTES 🔒 ------------ //
                    <ProtectedParentRoute
                        path=path!("/")
                        view=Outlet
                        condition=is_authenticated
                        redirect_path=|| "/login"
                        fallback=|| view! { <div class="p-4">"Checking authentication..."</div> }
                    >
                        <Route path=StaticSegment("/protected") view=ProtectedPage />
                        <Route path=StaticSegment("/only-root") view=OnlyRootPage />
                        <Route path=StaticSegment("/todos") view=PageTodos />
                    </ProtectedParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                         */
/* ========================================================== */

#[component]
pub fn PageHome() -> impl IntoView {
    view! { <h1>"Home"</h1> }
}
