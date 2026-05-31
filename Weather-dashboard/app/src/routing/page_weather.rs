use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::ui::button::Button;
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::ui::input::Input;

/* ========================================================== */
/*                     ✨ DATA TYPES ✨                       */
/* ========================================================== */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherData {
    pub city: String,
    pub country: String,
    pub temperature: f64,
    pub feels_like: f64,
    pub humidity: i32,
    pub wind_speed: f64,
    pub weather_code: i32,
}

fn weather_description(code: i32) -> &'static str {
    match code {
        0 => "Clear Sky",
        1 => "Mainly Clear",
        2 => "Partly Cloudy",
        3 => "Overcast",
        45 | 48 => "Foggy",
        51 | 53 | 55 => "Drizzle",
        56 | 57 => "Freezing Drizzle",
        61 | 63 | 65 => "Rain",
        66 | 67 => "Freezing Rain",
        71 | 73 | 75 => "Snowfall",
        77 => "Snow Grains",
        80 | 81 | 82 => "Rain Showers",
        85 | 86 => "Snow Showers",
        95 => "Thunderstorm",
        96 | 99 => "Thunderstorm with Hail",
        _ => "Unknown",
    }
}

fn weather_emoji(code: i32) -> &'static str {
    match code {
        0 => "☀️",
        1 | 2 => "🌤️",
        3 => "☁️",
        45 | 48 => "🌫️",
        51..=67 => "🌧️",
        71..=77 => "❄️",
        80..=82 => "🌦️",
        85 | 86 => "🌨️",
        95..=99 => "⛈️",
        _ => "🌡️",
    }
}

/* ========================================================== */
/*                   ✨ SERVER FUNCTION ✨                    */
/* ========================================================== */

/// Fetches current weather for a city using Open-Meteo (free, no API key).
/// Two steps: geocode city → fetch weather by lat/lon.
#[server]
pub async fn fetch_weather(city: String) -> Result<WeatherData, ServerFnError> {
    #[derive(serde::Deserialize)]
    struct GeoResponse {
        results: Option<Vec<GeoResult>>,
    }

    #[derive(serde::Deserialize)]
    struct GeoResult {
        name: String,
        latitude: f64,
        longitude: f64,
        country: String,
    }

    #[derive(serde::Deserialize)]
    struct WeatherApiResponse {
        current: CurrentWeather,
    }

    #[derive(serde::Deserialize)]
    struct CurrentWeather {
        temperature_2m: f64,
        apparent_temperature: f64,
        relative_humidity_2m: i32,
        wind_speed_10m: f64,
        weather_code: i32,
    }

    let client = reqwest::Client::new();

    // Step 1: resolve city name → coordinates
    let geo: GeoResponse = client
        .get("https://geocoding-api.open-meteo.com/v1/search")
        .query(&[
            ("name", city.as_str()),
            ("count", "1"),
            ("language", "en"),
            ("format", "json"),
        ])
        .send()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .json()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let location = geo
        .results
        .and_then(|r| r.into_iter().next())
        .ok_or_else(|| ServerFnError::new(format!("City '{}' not found", city)))?;

    // Step 2: fetch weather by coordinates
    let lat = location.latitude.to_string();
    let lon = location.longitude.to_string();

    let weather: WeatherApiResponse = client
        .get("https://api.open-meteo.com/v1/forecast")
        .query(&[
            ("latitude", lat.as_str()),
            ("longitude", lon.as_str()),
            (
                "current",
                "temperature_2m,apparent_temperature,relative_humidity_2m,wind_speed_10m,weather_code",
            ),
            ("wind_speed_unit", "kmh"),
        ])
        .send()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .json()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(WeatherData {
        city: location.name,
        country: location.country,
        temperature: weather.current.temperature_2m,
        feels_like: weather.current.apparent_temperature,
        humidity: weather.current.relative_humidity_2m,
        wind_speed: weather.current.wind_speed_10m,
        weather_code: weather.current.weather_code,
    })
}

/* ========================================================== */
/*                     ✨ PAGE ✨                             */
/* ========================================================== */

#[component]
pub fn WeatherPage() -> impl IntoView {
    let city_input = RwSignal::new(String::new());
    let searched_city = RwSignal::new("London".to_string());

    // Resource is SSR-aware: pre-renders weather data on the server for the initial
    // HTML response, then re-fetches on the client when searched_city changes.
    let weather_resource = Resource::new(move || searched_city.get(), |city| fetch_weather(city));

    let on_search = move |_| {
        let city = city_input.get();
        if !city.trim().is_empty() {
            searched_city.set(city.trim().to_string());
            city_input.set(String::new());
        }
    };

    view! {
        <div class="flex flex-col gap-8 p-6 max-w-xl mx-auto">
            <div class="flex gap-2">
                <Input
                    attr:placeholder="Search city (e.g. Tokyo, Paris...)"
                    on:input=move |ev| city_input.set(event_target_value(&ev))
                    prop:value=move || city_input.get()
                />
                <Button on:click=on_search>"Search"</Button>
            </div>

            <Suspense fallback=|| view! { <WeatherSkeleton /> }>
                {move || {
                    weather_resource
                        .get()
                        .map(|result| match result {
                            Ok(data) => view! { <WeatherDisplay data=data /> }.into_any(),
                            Err(err) => view! {
                                <div class="rounded-lg border border-destructive/30 bg-destructive/5 p-4 text-destructive text-sm">
                                    {format!("Could not load weather: {}", err)}
                                </div>
                            }
                            .into_any(),
                        })
                }}
            </Suspense>
        </div>
    }
}

/* ========================================================== */
/*                   ✨ COMPONENTS ✨                         */
/* ========================================================== */

#[component]
fn WeatherDisplay(data: WeatherData) -> impl IntoView {
    let emoji = weather_emoji(data.weather_code);
    let description = weather_description(data.weather_code);

    view! {
        <Card>
            <CardHeader>
                <div class="flex justify-between items-start">
                    <div>
                        <CardTitle>{format!("{}, {}", data.city, data.country)}</CardTitle>
                        <CardDescription>{description}</CardDescription>
                    </div>
                    <span class="text-5xl leading-none">{emoji}</span>
                </div>
            </CardHeader>
            <CardContent>
                <p class="text-6xl font-bold tracking-tight mt-2">
                    {format!("{:.1}°C", data.temperature)}
                </p>
                <p class="text-sm text-muted-foreground mt-1">
                    {format!("Feels like {:.1}°C", data.feels_like)}
                </p>

                <div class="grid grid-cols-2 gap-3 mt-6">
                    <WeatherStat label="Humidity" value=format!("{}%", data.humidity) icon="💧" />
                    <WeatherStat
                        label="Wind"
                        value=format!("{:.1} km/h", data.wind_speed)
                        icon="🌬️"
                    />
                </div>
            </CardContent>
        </Card>
    }
}

#[component]
fn WeatherStat(label: &'static str, value: String, icon: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1 rounded-md bg-muted/50 p-3">
            <span class="text-xs text-muted-foreground">{format!("{} {}", icon, label)}</span>
            <span class="text-base font-semibold">{value}</span>
        </div>
    }
}

#[component]
fn WeatherSkeleton() -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <div class="flex justify-between items-start">
                    <div class="flex flex-col gap-2">
                        <div class="h-6 bg-muted rounded animate-pulse w-40" />
                        <div class="h-4 bg-muted rounded animate-pulse w-24" />
                    </div>
                    <div class="size-12 bg-muted rounded animate-pulse" />
                </div>
            </CardHeader>
            <CardContent>
                <div class="h-14 bg-muted rounded animate-pulse w-36 mt-2" />
                <div class="h-4 bg-muted rounded animate-pulse w-28 mt-2" />
                <div class="grid grid-cols-2 gap-3 mt-6">
                    <div class="h-16 bg-muted rounded animate-pulse" />
                    <div class="h-16 bg-muted rounded animate-pulse" />
                </div>
            </CardContent>
        </Card>
    }
}
