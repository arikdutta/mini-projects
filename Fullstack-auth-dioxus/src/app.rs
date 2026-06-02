use dioxus::prelude::*;

use crate::domain::auth::_users::service::get_user::get_user;
use crate::domain::auth::routing::login_page::Login;
use crate::domain::auth::routing::only_root_page::OnlyRootPage;
use crate::domain::auth::routing::protected_page::Protected;
use crate::domain::todos::routing::todos_page::Todos;

const TAILWIND_CSS: Asset = asset!("/public/tailwind.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // Public routes (no layout guard)
    #[route("/login")]
    Login {},

    // Main layout wrapping all app pages
    #[layout(AppLayout)]
        #[route("/")]
        Home {},

        // Protected section — ProtectedLayout redirects to /login if unauthenticated
        #[layout(ProtectedLayout)]
            #[route("/protected")]
            Protected {},
            #[route("/todos")]
            Todos {},
            #[route("/only-root")]
            OnlyRootPage {},
        #[end_layout]
    #[end_layout]

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

// ── Layouts ───────────────────────────────────────────────────────────────────

#[component]
fn AppLayout() -> Element {
    rsx! {
        nav { class: "p-4 border-b flex gap-4 items-center",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Protected {}, "Protected" }
            Link { to: Route::Todos {}, "Todos" }
            Link { to: Route::OnlyRootPage {}, "Only Root" }
            Link { to: Route::Login {}, "Login" }
        }
        main { class: "p-4",
            Outlet::<Route> {}
        }
    }
}

/// Guards child routes. Redirects to /login if unauthenticated.
#[component]
fn ProtectedLayout() -> Element {
    let nav = use_navigator();
    let auth = use_resource(get_user);

    match auth.read().as_ref() {
        Some(Ok(Some(_user))) => rsx! { Outlet::<Route> {} },
        Some(Ok(None)) | Some(Err(_)) => {
            nav.push(Route::Login {});
            rsx! { div { "Redirecting to login..." } }
        }
        None => rsx! { div { "Checking auth..." } },
    }
}

// ── Pages ─────────────────────────────────────────────────────────────────────

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4",
            h1 { class: "text-2xl font-bold", "Home" }
            p { "Public page — no auth required." }
            Link { to: Route::Protected {}, "Go to protected page →" }
        }
    }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "p-8",
            p { class: "text-2xl font-semibold", "404" }
            p { "Page not found: /{route.join(\"/\")}" }
        }
    }
}
