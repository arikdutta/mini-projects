use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn PageDashboard() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-4xl font-bold tracking-tight">"DailyStack"</h1>
                <p class="text-muted-foreground mt-2">
                    "Your personal productivity hub — built with Leptos routing."
                </p>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <DashCard
                    href="/news?category=tech"
                    icon="📰"
                    title="News"
                    desc="Browse daily tech, Rust, and design headlines filtered by category."
                    badge="?category=tech"
                />
                <DashCard
                    href="/weather/london"
                    icon="🌤"
                    title="Weather"
                    desc="Check live conditions for any city via URL param."
                    badge="/weather/:city"
                />
                <DashCard
                    href="/tasks"
                    icon="✅"
                    title="Tasks"
                    desc="Track your to-dos with nested routes and an outlet panel."
                    badge="ParentRoute + Outlet"
                />
                <DashCard
                    href="/bookmarks?tag=dev"
                    icon="🔖"
                    title="Bookmarks"
                    desc="Quick-access saved links, filterable by tag."
                    badge="?tag=dev"
                />
            </div>
        </div>
    }
}

#[component]
fn DashCard(
    #[prop(into)] href: &'static str,
    #[prop(into)] icon: &'static str,
    #[prop(into)] title: &'static str,
    #[prop(into)] desc: &'static str,
    #[prop(into)] badge: &'static str,
) -> impl IntoView {
    view! {
        <A href=href>
            <div class="rounded-xl border bg-card p-5 hover:shadow-md hover:border-ring hover:bg-ring/10 transition-all cursor-pointer space-y-3">
                <div class="text-3xl">{icon}</div>
                <div>
                    <h2 class="text-lg font-semibold">{title}</h2>
                    <p class="text-sm text-muted-foreground mt-1">{desc}</p>
                </div>
                <span class="inline-block text-xs font-mono bg-muted px-2 py-1 rounded-md text-muted-foreground">
                    {badge}
                </span>
            </div>
        </A>
    }
}
