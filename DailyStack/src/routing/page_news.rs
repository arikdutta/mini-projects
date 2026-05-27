use leptos::prelude::*;
use leptos_router::components::A;

use crate::utils::query::{QueryUtils, QUERY};

struct NewsItem {
    title: &'static str,
    source: &'static str,
    category: &'static str,
    summary: &'static str,
}

fn mock_news() -> Vec<NewsItem> {
    vec![
        NewsItem {
            title: "Rust 2024 Edition Released",
            source: "blog.rust-lang.org",
            category: "rust",
            summary: "The 2024 edition brings new idioms, improved ergonomics, and better async patterns.",
        },
        NewsItem {
            title: "Leptos 0.8 Ships with Typed Routes",
            source: "leptos.dev",
            category: "rust",
            summary: "Leptos 0.8 introduces a new typed routing API using segment types and nightly features.",
        },
        NewsItem {
            title: "cargo-semver-checks v0.40",
            source: "crates.io",
            category: "rust",
            summary: "New lint rules for detecting breaking changes in public Rust APIs automatically.",
        },
        NewsItem {
            title: "WebAssembly Gets Garbage Collection",
            source: "webassembly.org",
            category: "tech",
            summary: "Wasm GC proposal lands in major browsers, enabling high-level language compilation to Wasm.",
        },
        NewsItem {
            title: "Tailwind CSS v4 Released",
            source: "tailwindcss.com",
            category: "tech",
            summary: "Tailwind v4 brings a new engine, CSS-first configuration, and major performance gains.",
        },
        NewsItem {
            title: "AI Code Tools Benchmark 2025",
            source: "github.blog",
            category: "tech",
            summary: "GitHub's annual benchmark of AI coding assistants across real-world software tasks.",
        },
        NewsItem {
            title: "Figma Variables Go Generally Available",
            source: "figma.com",
            category: "design",
            summary: "Variables in Figma are now GA, enabling dynamic design tokens and component theming.",
        },
        NewsItem {
            title: "Design Systems at Scale in 2025",
            source: "smashingmagazine.com",
            category: "design",
            summary: "How teams are scaling design systems with tokens, documentation, and governance.",
        },
    ]
}

#[component]
pub fn PageNews() -> impl IntoView {
    let category = QueryUtils::extract(QUERY::CATEGORY.to_string());

    let filtered = move || {
        let cat = category.get();
        mock_news()
            .into_iter()
            .filter(|n| cat.is_empty() || n.category == cat)
            .collect::<Vec<_>>()
    };

    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-2xl font-bold">"📰 News Feed"</h1>
                <p class="text-muted-foreground text-sm mt-1">
                    "Query: "
                    <span class="font-mono bg-muted px-1.5 py-0.5 rounded text-xs">
                        {move || {
                            let c = category.get();
                            if c.is_empty() { "all".to_string() } else { c }
                        }}
                    </span>
                </p>
            </div>

            <div class="flex gap-2 flex-wrap">
                <FilterPill href="/news" label="All" />
                <FilterPill href="/news?category=tech" label="Tech" />
                <FilterPill href="/news?category=rust" label="Rust" />
                <FilterPill href="/news?category=design" label="Design" />
            </div>

            <div class="space-y-3">
                {move || {
                    filtered()
                        .into_iter()
                        .map(|item| {
                            view! {
                                <div class="rounded-lg border bg-card p-4 space-y-1.5 hover:border-ring hover:bg-ring/10 transition-all">
                                    <div class="flex items-start justify-between gap-2">
                                        <h3 class="font-semibold leading-snug">{item.title}</h3>
                                        <span class="shrink-0 text-xs bg-muted px-2 py-0.5 rounded-full font-mono text-muted-foreground">
                                            {item.category}
                                        </span>
                                    </div>
                                    <p class="text-sm text-muted-foreground">{item.summary}</p>
                                    <p class="text-xs text-muted-foreground/60">{item.source}</p>
                                </div>
                            }
                        })
                        .collect_view()
                }}
            </div>
        </div>
    }
}

#[component]
fn FilterPill(#[prop(into)] href: &'static str, #[prop(into)] label: &'static str) -> impl IntoView {
    view! {
        <A href=href>
            <span class="px-3 py-1 rounded-full border text-sm hover:bg-accent transition-colors cursor-pointer">
                {label}
            </span>
        </A>
    }
}
