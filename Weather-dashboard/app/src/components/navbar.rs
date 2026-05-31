use leptos::prelude::*;

use super::reactive_indicator::ReactiveIndicator;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="flex sticky top-0 z-50 justify-between items-center p-4 bg-background border-b border-border">
            <span class="font-semibold text-lg mb-4">"Weather Dashboard"</span>
            <div class="flex gap-4 items-center">
                <ReactiveIndicator />
            </div>
        </div>
    }
}
