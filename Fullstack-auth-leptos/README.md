# Fullstack Auth — Leptos

A full-stack authentication and authorization template built with **Leptos 0.8**, **Axum**, and **PostgreSQL**. Demonstrates server-side rendering with WASM hydration, role-based access control, session management, and a CRUD todos feature — all in Rust.

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Full-stack framework | Leptos 0.8 (nightly) |
| Web server | Axum 0.8 |
| Database | PostgreSQL + SQLx 0.8 |
| Async runtime | Tokio |
| Session management | axum_session_auth 0.16 |
| CSS | Tailwind CSS 4.1 |
| UI components | Leptos UI 0.3 |
| Password hashing | pwhash (crypt) |
| Rust edition | 2024 (nightly) |

---

## Features

- **Hybrid SSR + hydration** — server renders HTML, client hydrates WASM for interactivity
- **Session-based authentication** — sessions stored in PostgreSQL via `axum_session_auth`
- **Role-based access control** — `Root`, `Admin`, `Staff`, `RegularUser` roles with a `#[secured_server]` proc macro for permission-checked server functions
- **Protected routes** — `ProtectedParentRoute` redirects unauthenticated users to `/login`
- **Todos CRUD** — create, list, and delete todos via typed server functions
- **Compile-time SQL** — SQLx with type-checked queries
- **Auto migrations** — SQLx migrations run on server startup
- **Custom Tailwind dark mode** — OKLch color scheme with tw-animate-css

---

## Project Structure

```
Fullstack-auth-leptos/
├── app/                  # Isomorphic Leptos library (runs on server + client)
│   └── src/
│       ├── app.rs        # Root component and router
│       ├── shell.rs      # HTML shell with meta tags
│       ├── common/       # AppState, error types, error template
│       ├── components/   # UI primitives (Button, Card, Input, Dialog, Table, Toast)
│       ├── domain/
│       │   ├── auth/     # User entity, roles, permissions, login/logout server fns
│       │   └── todos/    # Todos server fns, DB access, and page components
│       ├── shared/       # Route definitions, NotFound page
│       └── utils/        # Identifiable trait, Unid wrapper
├── server/               # Axum binary — bootstraps DB, runs migrations, starts server
├── crates/macros/        # #[secured_server] procedural macro
├── migrations/           # SQLx migration files
├── style/                # Tailwind CSS entry point
├── public/               # Static assets
├── tests/                # Integration test crate
├── rust-toolchain.toml   # Pins nightly Rust
└── .env                  # Environment variables (see setup below)
```

---

## Routes

| Path | Access | Description |
|------|--------|-------------|
| `/` | Public | Home page |
| `/login` | Public | Login form |
| `/test` | Public | Test page |
| `/protected` | Authenticated | Example protected page |
| `/only-root` | Root role only | Restricted page |
| `/todos` | Authenticated | Todos CRUD |

---

## Getting Started

### Prerequisites

- **Rust nightly** (managed by `rust-toolchain.toml`)
- **PostgreSQL** running locally
- **cargo-leptos** — install with:
  ```bash
  cargo install cargo-leptos
  ```
- **Node.js** — for Tailwind CSS CLI:
  ```bash
  npm install
  ```
- **WASM target:**
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

### Environment Setup

Edit `.env` with your database credentials:

```env
LEPTOS_SITE_ADDR=127.0.0.1:3000
DATABASE_URL=postgres://user:password@localhost/dbname?options=-c%20search_path%3Dapp_schema
APP_SCHEMA=app_schema
SESSION_TABLE_NAME=_app_sessions
```

### Database

Create the PostgreSQL database, then let the server run migrations automatically on first startup.

### Run in Development

```bash
cargo leptos watch
```

The app will be available at `http://127.0.0.1:3000`.

### Build for Production

```bash
cargo leptos build --release
```

---

## Test Users

The following seed users are available after running migrations:

| Role | Email | Password |
|------|-------|----------|
| Root | root@example.com | `password` |
| Admin | admin@example.com | `password` |
| RegularUser | alice@example.com | `password` |
| RegularUser | bob@example.com | `password` |

---

## Authentication Flow

1. User submits credentials at `/login`
2. Server validates password with `pwhash::unix::verify`
3. On success, `axum_session_auth` stores the session in PostgreSQL
4. Auth middleware reads the session on each request and redirects unauthenticated users to `/login`
5. The current user is injected into the Leptos context via `provide_context` in Axum handlers
6. Server functions access the user through the injected `AuthSession`

---

## Role & Permission System

Roles are defined in `app/src/domain/auth/_users/role.rs`:

```rust
pub enum Role {
    Root,
    Admin,
    Staff,
    RegularUser,
}
```

The `#[secured_server]` macro (in `crates/macros`) wraps server functions with a permission check, returning `Unauthorized` if the caller lacks the required role.

---

## Workspace Members

| Crate | Purpose |
|-------|---------|
| `app` | Shared isomorphic code — features `csr`, `hydrate`, `ssr` |
| `server` | Axum binary with SSR feature enabled |
| `crates/macros` | `#[secured_server]` proc macro |
| `tests` | Integration tests |

---

## Development Tools

| Tool | Command |
|------|---------|
| Format Rust | `cargo fmt` |
| Format Leptos views | `leptosfmt .` |
| Lint JS | `npx biome check .` |
| Run tests | `cargo test` |
