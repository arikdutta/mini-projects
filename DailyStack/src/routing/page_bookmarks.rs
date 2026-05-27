use leptos::prelude::*;
use leptos_router::components::A;

use crate::utils::query::{QueryUtils, QUERY};

struct Bookmark {
    title: &'static str,
    url: &'static str,
    tag: &'static str,
    desc: &'static str,
}

fn mock_bookmarks() -> Vec<Bookmark> {
    vec![
        Bookmark { title: "Leptos Book", url: "leptos.dev/docs", tag: "dev", desc: "Official Leptos framework documentation and guides." },
        Bookmark { title: "Rust by Example", url: "doc.rust-lang.org/rust-by-example", tag: "dev", desc: "Learn Rust through annotated example programs." },
        Bookmark { title: "Rustonomicon", url: "doc.rust-lang.org/nomicon", tag: "dev", desc: "The dark arts of advanced and unsafe Rust programming." },
        Bookmark { title: "Tailwind v4 Docs", url: "tailwindcss.com/docs", tag: "tools", desc: "New CSS-first configuration and full utility class reference." },
        Bookmark { title: "Trunk CLI", url: "trunkrs.dev", tag: "tools", desc: "WASM-based web application bundler for Rust." },
        Bookmark { title: "crates.io", url: "crates.io", tag: "tools", desc: "The official Rust community package registry." },
        Bookmark { title: "WebAssembly MDN Guide", url: "developer.mozilla.org/wasm", tag: "articles", desc: "MDN reference for WebAssembly APIs and browser integration." },
        Bookmark { title: "Component-Driven UI", url: "componentdriven.org", tag: "articles", desc: "Principles and practices for building scalable component systems." },
    ]
}

#[component]
pub fn PageBookmarks() -> impl IntoView {
    let tag = QueryUtils::extract(QUERY::TAG.to_string());

    let filtered = move || {
        let t = tag.get();
        mock_bookmarks()
            .into_iter()
            .filter(|b| t.is_empty() || b.tag == t)
            .collect::<Vec<_>>()
    };

    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-2xl font-bold">"🔖 Bookmarks"</h1>
                <p class="text-muted-foreground text-sm mt-1">
                    "Query: "
                    <span class="font-mono bg-muted px-1.5 py-0.5 rounded text-xs">
                        {move || {
                            let t = tag.get();
                            if t.is_empty() { "all".to_string() } else { t }
                        }}
                    </span>
                </p>
            </div>

            <div class="flex gap-2 flex-wrap">
                <FilterPill href="/bookmarks" label="All" />
                <FilterPill href="/bookmarks?tag=dev" label="Dev" />
                <FilterPill href="/bookmarks?tag=tools" label="Tools" />
                <FilterPill href="/bookmarks?tag=articles" label="Articles" />
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                {move || {
                    filtered()
                        .into_iter()
                        .map(|bm| {
                            view! {
                                <div class="rounded-lg border bg-card p-4 space-y-1.5 hover:border-ring hover:bg-ring/10 transition-all">
                                    <div class="flex items-start justify-between gap-2">
                                        <h3 class="font-semibold text-sm">{bm.title}</h3>
                                        <span class="shrink-0 text-xs bg-muted px-2 py-0.5 rounded-full font-mono text-muted-foreground">
                                            {bm.tag}
                                        </span>
                                    </div>
                                    <p class="text-sm text-muted-foreground">{bm.desc}</p>
                                    <p class="text-xs font-mono text-muted-foreground/60">{bm.url}</p>
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
