use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    ParamSegment, StaticSegment,
};

use crate::{
    components::header::Header,
    domain::user::routing::{user_layout::UserLayout, user_profile_page::UserProfilePage},
    routing::{page_home::PageHome, page_param::PageParam, page_query::PageQuery},
    utils::param::PARAM,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Header />

            <main class="min-h-screen">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("/") view=PageHome />

                    // ------- PARENT ROUTE ---------
                    <ParentRoute path=StaticSegment("users") view=UserLayout>
                        <Route path=StaticSegment("") view=|| view! { <p>"Select a user to view profile"</p> } />
                        <Route path=StaticSegment("profile") view=UserProfilePage />
                    </ParentRoute>

                    // ------- QUERY & PARAM ---------
                    <Route path=StaticSegment("/page-query") view=PageQuery />
                    <Route path=(StaticSegment("/page-param"), ParamSegment(PARAM::NAME)) view=PageParam />

                // <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> } />
                </Routes>
            </main>
        </Router>
    }
}
