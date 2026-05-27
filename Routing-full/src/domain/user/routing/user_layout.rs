use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn UserLayout() -> impl IntoView {
    const OUTLET_WRAPPER: &str = "p-4 mt-4 rounded-md border-2 border-sky-500 border-dashed";

    view! {
        <div class="flex flex-col gap-4 p-4">
            <h1 class="text-2xl font-bold">"User Layout"</h1>

            <div class=OUTLET_WRAPPER>
                <span class="text-sky-500">"Outlet"</span>
                <Outlet />
            </div>
        </div>
    }
}
