use leptos::prelude::*;
use leptos_router::hooks::use_location;
use web_sys::window;

use crate::components::ui::button::Button;
use crate::domain::auth::_users::service::get_user::get_user;
use crate::domain::auth::_users::service::logout::Logout;

#[component]
pub fn Header() -> impl IntoView {
    let location = use_location();
    let logout = ServerAction::<Logout>::new();

    let user_resource = Resource::new(
        move || (logout.version().get(), location.pathname.get()),
        move |_| get_user(),
    );

    let is_user_logged_in = move || matches!(user_resource.get(), Some(Ok(Some(_))));

    view! {
        <header class="flex sticky top-0 flex-col gap-4 pt-4 pb-6 border-b z-100 bg-accent">
            <nav class="flex items-center lg:justify-between xl:px-20">
                <div class="flex gap-4 items-center">
                    <MenuLink href="/">"Home"</MenuLink>
                    <MenuLink href="/test">"Test"</MenuLink>
                    <MenuLink href="/protected">"Protected"</MenuLink>
                    <MenuLink href="/only-root">"Only Root"</MenuLink>
                    <MenuLink href="/todos">"Todos"</MenuLink>
                </div>

                <div class="flex gap-4 justify-end items-center max-lg:ml-auto">
                    <Transition fallback=move || {
                        view! { <span>"Loading..."</span> }
                    }>

                        {move || {
                            user_resource
                                .get()
                                .map(|user| match user {
                                    Err(_err) => view! { <div>"Error loading user: 👉 TODO"</div> }.into_any(),
                                    Ok(None) => view! { "" }.into_any(),
                                    Ok(Some(_user)) => {
                                        view! { <div class="rounded-full size-10 bg-neutral-200" /> }.into_any()
                                    }
                                })
                                .or_else(|| { Some(view! { "" }.into_any()) })
                        }}
                    </Transition>

                    <Transition>
                        <Show when=move || !is_user_logged_in()>
                            <button
                                on:click=move |_| {
                                    if let Some(window) = window() {
                                        let _ = window.location().set_href("/login");
                                    }
                                }
                                class="inline-flex justify-center items-center py-2 px-4 h-9 text-sm font-medium whitespace-nowrap rounded-md transition-colors focus-visible:ring-1 disabled:opacity-50 disabled:pointer-events-none w-fit bg-primary text-primary-foreground hover:bg-primary/90 focus-visible:outline-hidden focus-visible:ring-ring"
                            >
                                "Login"
                            </button>
                        </Show>
                    </Transition>

                    <Transition>
                        <Show when=is_user_logged_in>
                            <ActionForm action=logout>
                                <Button attr:r#type="submit">"Log Out"</Button>
                            </ActionForm>
                        </Show>
                    </Transition>
                </div>
            </nav>
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

    let navigate = move |_| {
        if let Some(window) = window() {
            let _ = window.location().set_href(href);
        }
    };

    view! {
        <button class:font-bold=move || is_active.get() on:click=navigate class="cursor-pointer">
            {children()}
        </button>
    }
}
