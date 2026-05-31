#![recursion_limit = "256"]

use app::app::App;
use app::shell::shell;
use axum::Router;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};

#[tokio::main]
async fn main() {
    let conf = get_configuration(None).expect("failed to read Leptos configuration");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("failed to bind address");
    axum::serve(listener, app.into_make_service()).await.expect("server error");
}
