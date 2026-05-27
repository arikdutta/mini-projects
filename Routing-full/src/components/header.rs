use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};
use leptos_ui::clx;

#[component]
pub fn Header() -> impl IntoView {
    clx! {Navbar, nav, "flex gap-2 p-4 "}
    // a! {MenuLink, "px-4 py-2 rounded-md bg-accent"}

    view! {
        <header>
            <Navbar>
                <MenuLink href="/">"Home"</MenuLink>
                <MenuLink href="/users">"Users"</MenuLink>
                <MenuLink href="/users/profile">"User Profile"</MenuLink>
                <MenuLink href="/page-param/123">"Param Example"</MenuLink>
                <MenuLink href="/page-query?demo=example">"Query Example"</MenuLink>
            </Navbar>
        </header>
    }
}


/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                         */
/* ========================================================== */

#[component]
pub fn MenuLink(#[prop(into)] href: &'static str, children: Children) -> impl IntoView {
    let location = use_location();
    let is_active = Memo::new(move |_| location.pathname.get() == href);

    view! {
        <A class:font-bold=move || is_active.get() href=href>
            <span class="py-2 px-4 rounded-md bg-accent">{children()}</span>
        </A>
    }
}
