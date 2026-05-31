use leptos::prelude::*;
use leptos_ui::void;

const TIMEOUT_MS: u64 = 100;

#[component]
pub fn ReactiveIndicator() -> impl IntoView {
    let is_reactive = RwSignal::new(false);

    void! {ReactiveIndicatorState, div, "size-3 rounded-full transition-colors duration-300 ease-in-out"}

    Effect::new(move |_| {
        set_timeout(
            move || {
                is_reactive.set(true);
            },
            std::time::Duration::from_millis(TIMEOUT_MS),
        );
    });

    view! {
        <ReactiveIndicatorState
            class:bg-green-500=move || is_reactive.get()
            class:bg-orange-500=move || !is_reactive.get()
        />
    }
}
