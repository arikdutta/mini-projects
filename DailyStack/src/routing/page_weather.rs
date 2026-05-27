use leptos::prelude::*;

use crate::utils::param::{ParamsUtils, PARAM};

#[derive(PartialEq, Clone)]
struct WeatherData {
    city: &'static str,
    temp: &'static str,
    condition: &'static str,
    humidity: &'static str,
    wind: &'static str,
    icon: &'static str,
}

fn mock_weather(city: &str) -> WeatherData {
    match city.to_lowercase().as_str() {
        "barcelona" => WeatherData { city: "Barcelona", temp: "26°C", condition: "Partly Cloudy", humidity: "72%", wind: "18 km/h", icon: "🌥" },
        "london" => WeatherData { city: "London", temp: "14°C", condition: "Partly Cloudy", humidity: "72%", wind: "18 km/h", icon: "🌥" },
        "paris" => WeatherData { city: "Paris", temp: "17°C", condition: "Sunny", humidity: "58%", wind: "12 km/h", icon: "☀️" },
        "tokyo" => WeatherData { city: "Tokyo", temp: "23°C", condition: "Humid & Warm", humidity: "85%", wind: "8 km/h", icon: "🌤" },
        "newyork" => WeatherData { city: "New York", temp: "19°C", condition: "Clear", humidity: "60%", wind: "22 km/h", icon: "🌞" },
        "berlin" => WeatherData { city: "Berlin", temp: "11°C", condition: "Overcast", humidity: "80%", wind: "15 km/h", icon: "☁️" },
        _ => WeatherData { city: "Unknown City", temp: "--", condition: "No weather data for this city", humidity: "--", wind: "--", icon: "❓" },
    }
}

const CITIES: &[&str] = &["barcelona", "london", "paris", "tokyo", "newyork", "berlin"];

#[component]
pub fn PageWeather() -> impl IntoView {
    let city = ParamsUtils::extract(PARAM::CITY.to_string());

    let weather = Memo::new(move |_| mock_weather(&city.get().unwrap_or_default()));

    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-2xl font-bold">"🌤 Weather"</h1>
                <p class="text-muted-foreground text-sm mt-1">
                    "Route param: "
                    <span class="font-mono bg-muted px-1.5 py-0.5 rounded text-xs">
                        {move || format!("/weather/{}", city.get().unwrap_or_default())}
                    </span>
                </p>
            </div>

            <div class="rounded-xl border bg-card p-6 space-y-5 max-w-sm">
                <div class="flex items-center gap-4">
                    <span class="text-5xl">{move || weather.get().icon}</span>
                    <div>
                        <h2 class="text-xl font-bold">{move || weather.get().city}</h2>
                        <p class="text-muted-foreground text-sm">{move || weather.get().condition}</p>
                    </div>
                </div>

                <div class="text-5xl font-bold tracking-tight">{move || weather.get().temp}</div>

                <div class="grid grid-cols-2 gap-3">
                    <div class="rounded-lg bg-muted px-3 py-2">
                        <p class="text-xs text-muted-foreground">"Humidity"</p>
                        <p class="font-semibold text-sm mt-0.5">{move || weather.get().humidity}</p>
                    </div>
                    <div class="rounded-lg bg-muted px-3 py-2">
                        <p class="text-xs text-muted-foreground">"Wind"</p>
                        <p class="font-semibold text-sm mt-0.5">{move || weather.get().wind}</p>
                    </div>
                </div>
            </div>

            <div>
                <p class="text-sm text-muted-foreground mb-2">"Try another city:"</p>
                <div class="flex gap-2 flex-wrap">
                    {CITIES
                        .iter()
                        .map(|c| {
                            let href = format!("/weather/{c}");
                            view! {
                                <a
                                    href=href
                                    class="px-3 py-1 rounded-full border text-sm hover:bg-accent transition-colors capitalize"
                                >
                                    {*c}
                                </a>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </div>
    }
}
