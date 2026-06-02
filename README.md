# mini-projects

[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Leptos](https://img.shields.io/badge/Leptos-EF3939?logo=rust&logoColor=white)](https://leptos.dev/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A hands-on, day-by-day journey through building web applications in Rust with the [Leptos](https://leptos.dev/) framework. Each day is a self-contained module with focused exercises that build on the previous ones.

## What's Inside

Each day lives in its own directory and contains a short guide plus a set of progressive exercises. Concepts start simple and compound over time, so the modules are best worked through in order.

A representative module:

### Day 18 — Routing

Master client-side routing in Leptos to build multi-page single-page applications with navigation, params, and queries.

**Learning objectives**

- Set up the Leptos Router for client-side navigation
- Create routes and navigate between pages
- Implement nested routes and parent layouts
- Work with route parameters and dynamic paths
- Handle query parameters and search strings
- Build a complete multi-page application with routing

**Exercises**

| Exercise | Focus |
| --- | --- |
| `01-basics` | Basic routing setup and navigation |
| `02-parent-route` | Nested routes with parent layouts |
| `03-param` | Dynamic route parameters (e.g. `/users/:id`) |
| `04-query` | Query string parameters (e.g. `/search?q=rust`) |
| `05-routing-full` | Complete routing app combining all concepts |

**Key concepts**

- **Router** — client-side navigation without page reloads
- **Routes** — URL patterns mapped to components
- **Nested routes** — routes within routes for shared layouts
- **Route parameters** — dynamic URL segments (`:id`, `:slug`)
- **Query parameters** — URL search strings (`?key=value`)
- **Navigation** — programmatic and declarative routing

## Getting Started

### Prerequisites

- A recent [Rust toolchain](https://www.rust-lang.org/tools/install) (install via `rustup`)
- The WebAssembly target: `rustup target add wasm32-unknown-unknown`
- [Trunk](https://trunkrs.dev/) for building and serving the apps: `cargo install trunk`

### Running an exercise

```bash
# Move into the exercise you want to run
cd day-18-routing/01-basics

# Build and serve with hot reload
trunk serve --open
```

The app will be available at `http://localhost:8080` by default.

## Repository Layout

```
mini-projects/
├── day-18-routing/
│   ├── 01-basics/
│   ├── 02-parent-route/
│   ├── 03-param/
│   ├── 04-query/
│   └── 05-routing-full/
└── README.md
```

## Contributing

Contributions, fixes, and suggestions are welcome. Feel free to open an issue or submit a pull request.

## License

Released under the MIT License. See [`LICENSE`](LICENSE) for details.
