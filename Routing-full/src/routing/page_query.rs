use leptos::prelude::*;

use crate::utils::query::{QueryUtils, QUERY};

// http://localhost:3000/page-query?demo=coucou

#[component]
pub fn PageQuery() -> impl IntoView {
    let demo_query = QueryUtils::extract(QUERY::DEMO.to_string());

    view! {
        <div>
            <h1>"Home Page"</h1>
            <p>"Demo Query: " {demo_query}</p>
        </div>
    }
}
