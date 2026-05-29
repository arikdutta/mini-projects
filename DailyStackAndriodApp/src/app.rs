use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    ParamSegment, StaticSegment,
};

use crate::{
    components::header::Header,
    domain::tasks::routing::tasks_layout::TasksLayout,
    routing::{
        page_bookmarks::PageBookmarks, page_dashboard::PageDashboard, page_news::PageNews,
        page_weather::PageWeather,
    },
    utils::param::PARAM,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Header />

            <main class="min-h-screen max-w-5xl mx-auto px-4 py-8">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("/") view=PageDashboard />
                    <Route path=StaticSegment("/news") view=PageNews />
                    <Route path=(StaticSegment("/weather"), ParamSegment(PARAM::CITY)) view=PageWeather />

                    <Route path=StaticSegment("/tasks") view=TasksLayout />

                    <Route path=StaticSegment("/bookmarks") view=PageBookmarks />
                </Routes>
            </main>
        </Router>
    }
}
