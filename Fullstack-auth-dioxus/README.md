# Fullstack Auth — Dioxus

A full-stack authentication demo built entirely in Rust using Dioxus 0.7, Axum, and PostgreSQL. Demonstrates role-based access control, session management, and permission enforcement via a custom procedural macro.

## Tech Stack

| Layer | Technology |
|---|---|
| Frontend | Dioxus 0.7 (React-like, compiles to WASM) |
| Backend | Axum (embedded via `dioxus serve`) |
| Database | PostgreSQL + SQLx (compile-time checked queries) |
| Sessions | `axum_session` + `axum_session_sqlx` (DB-backed) |
| Auth | `axum_session_auth` |
| Password Hashing | `pwhash` (UNIX crypt) |
| Styling | Tailwind CSS + `tw_merge` |

## Features

- Full-stack auth in pure Rust (no JS auth logic)
- Role-based access control: `Root`, `Admin`, `Staff`, `RegularUser`
- Protected routes with automatic redirect to `/login`
- PostgreSQL-persisted sessions
- `#[secured_server]` — custom proc macro that injects auth + permission checks into server functions
- Component-based UI built with reusable Tailwind-styled primitives (Button, Card, Input, Alert, etc.)
- SQLx migrations with seeded test data

## Project Structure

```
Fullstack-auth-dioxus/
├── crates/
│   └── macros/              # #[secured_server] procedural macro
├── migrations/              # PostgreSQL migrations + seed data
├── public/                  # Static assets
├── src/
│   ├── components/ui/       # Reusable UI components
│   ├── domain/
│   │   ├── auth/            # User model, login/logout, roles, permissions
│   │   └── todos/           # Todo feature (protected route example)
│   ├── app.rs               # Router, layouts, protected route guard
│   └── main.rs              # Entry point (client + server modes)
├── style/                   # Tailwind CSS source
├── .env                     # Database + session config
└── Dioxus.toml
```

## Routes

| Path | Access | Description |
|---|---|---|
| `/login` | Public | Login form |
| `/` | Authenticated | Home page |
| `/protected` | Authenticated | Shows current user's name and email |
| `/todos` | Authenticated | Todo list |
| `/only-root` | `Root` role only | Lists all users; enforced by `#[secured_server]` |

## `#[secured_server]` Macro

A drop-in replacement for Dioxus's `#[server]` that automatically injects session extraction, user resolution, and permission checks before the function body runs.

```rust
#[secured_server]
pub async fn get_root_data() -> Result<RootData, ServerFnError> {
    // body only runs if check_permission passes
}

impl AccessController for GetRootDataController {
    fn check_permission(user: &User) -> Result<(), AppError> {
        OnlyRootPermission::Access.check_permission(user)
    }
}
```

## Prerequisites

- Rust (see `rust-toolchain.toml` for the required toolchain)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started) (`cargo install dioxus-cli`)
- PostgreSQL running locally

## Setup

**1. Configure the database**

Create a `.env` file (or update the existing one):

```env
POSTGRES_USER=postgres
POSTGRES_PASSWORD=password
POSTGRES_DATABASE=fullstack_auth_dioxus
APP_SCHEMA=app_schema
DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@127.0.0.1/${POSTGRES_DATABASE}?options=-c%20search_path=public,${APP_SCHEMA}
SESSION_TABLE_NAME=_app_sessions
```

**2. Run migrations**

Migrations run automatically on server startup via `sqlx::migrate!()`. They create the schema, tables, and seed test users.

**3. Start the app**

```bash
dx serve --features server
```

The app runs at `http://localhost:8080` by default.
