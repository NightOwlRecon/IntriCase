# Project Layout

Front-end code (TypeScript/Svelte/Tailwind) is in the `ui` directory.

Back-end (Rust/axum/sqlx) code is in the `src` directory.

PostgreSQL migrations are in the `migrations` directory.

Templates (Handlebars) for email and SMS notifications are in the `templates` directory.

## Back-end Layout

The back-end is broken up between `core` and `api` modules.  Core contains the application itself, and `api` exposes a subset of that functionality as a REST API for the front-end to interact with.

The hierarchy of the back-end's `api` module and submodules should match the hierarchy of the REST endpoints they expose.  Each module has its own axum router, with any submodules having their own nested routers as well.

This may not be the best way to structure the back-end as it grows, but for now, the code behind each API endpoint is encapsulated in a single space.

The `api` module takes HTTP requests and calls the relevant underlying functions in `core`,
translating the results to HTTP responses and response codes. It also handles reading and writing cookies, as well as additional server-side form validation.

The `core` module is where the "meat" of the application lives. Everything in this module that is triggered by a web request should be called only by request handlers in the `api` module; no `axum` routers or otherwise should exist in `core`.

To install the sqlx CLI, run the following after installing Rust: `cargo install sqlx-cli --no-default-features --features postgres,rustls`

To create a new migration, run `sqlx migrate add <MIGRATION_NAME>`

## Front-end Layout

Routes (using the anchor tag) live in `App.svelte`

TypeScript bindings are generated from Rust structs by running `cargo check`, and live in `ui/src/bindings`.  In the future, these could theoretically follow a similar approach to the back-end instead of being defined in a single place.

