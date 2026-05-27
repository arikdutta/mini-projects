use leptos::prelude::*;

use crate::utils::param::{ParamsUtils, PARAM};

#[component]
pub fn PageParam() -> impl IntoView {
    let params_demo_name = ParamsUtils::extract(PARAM::NAME.to_string());

    view! {
        <div>
            <h1>"Parameter Page"</h1>
            <p>"Param: " {params_demo_name}</p>
        </div>
    }
}
