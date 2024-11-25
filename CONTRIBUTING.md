# Contributing to IceBlink

## App

1. Move into the `app` folder
2. ??? Develop the app maybe

## Sync server

1. Move into `sync` folder
2. Install the Rust installation manager `rustup`
3. Enable Rust nightly with `rustup default nightly`
4. Install the sqlx CLI using `cargo install sqlx-cli` - this will take some time
5. Copy `.env.example` to `.env`
6. Update using values from an IdP of your choice
7. Setup the SQLite database with `sqlx database setup`
8. Serve using `cargo run -- serve`

The project is optimized for faster compiletime in dev, and faster runtime at
release. Release builds made with `--release` will compile slower due to theese
runtime performance optimizations. **Yes we know the compile times are slow,
there is little we can do**

An OpenAPI Swagger UI is available at `/swagger` with the OpenAPI spec at
`/openapi.json`. These are generated using the `utoipa` family of crates,
specifically `utoipa`, `utoipa-axum` and `utoipa-swagger-ui`. These do not make
a perfect specification, but it's a good overview of the endpoints. For further
information about the endpoints you can look into the code.

### Testing

Tests can be run with `cargo test`. Unit tests test specific small pieces of
code, and should live next to the source-code. These tests work well for
utilities. Integration tests are in the `tests` folder, and test it as if it is
an HTTP API. By using the `sqlx::text` macro we get passed a mock database into
the test function. This database is reset, but applied with the data from
fixtures.
