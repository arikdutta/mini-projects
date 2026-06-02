use leptos::prelude::*;
use leptos_meta::MetaTags;

use crate::app::App;

pub fn shell(leptos_options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <title text="Title" />
                <link rel="shortcut icon" type_="image/png" href="/favicon.png" />
                <link id="leptos" href="/pkg/w5_d23_fullstack_auth.css" rel="stylesheet" />
                <AutoReload options=leptos_options.clone() />
                <HydrationScripts options=leptos_options />
                <MetaTags />
            </head>

            <body>
                <App />
            </body>
        </html>
    }
}
