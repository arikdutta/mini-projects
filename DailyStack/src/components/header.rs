use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};
use leptos_ui::clx;

#[component]
pub fn Header() -> impl IntoView {
    clx! {Navbar, nav, "flex gap-2 p-4 border-b bg-card flex-wrap items-center"}

    view! {
        <header>
            <Navbar>
                <MenuLink href="/" active_path="/">"Dashboard"</MenuLink>
                <MenuLink href="/news?category=tech" active_path="/news">"News"</MenuLink>
                <MenuLink href="/weather/barcelona" active_path="/weather">"Weather"</MenuLink>
                <MenuLink href="/tasks" active_path="/tasks">"Tasks"</MenuLink>
                <MenuLink href="/bookmarks?tag=dev" active_path="/bookmarks">"Bookmarks"</MenuLink>
            </Navbar>
        </header>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                         */
/* ========================================================== */

#[component]
pub fn MenuLink(
    #[prop(into)] href: &'static str,
    #[prop(into)] active_path: &'static str,
    children: Children,
) -> impl IntoView {
    let location = use_location();

    let is_active = Memo::new(move |_| {
        let path = location.pathname.get();
        if active_path == "/" { path == "/" } else { path.starts_with(active_path) }
    });

    view! {
        <A class:font-bold=move || is_active.get() href=href>
            <span class="py-2 px-4 rounded-md bg-accent hover:bg-accent/80 transition-colors text-sm">
                {children()}
            </span>
        </A>
    }
}
