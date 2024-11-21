# Contributing to IceBlink

## App

1. Move into the `app` folder

## Sync server

1. Move into `sync` folder
2. Install a recent version of Rust using `rustup`
3. Install the sqlx CLI using `cargo install sqlx-cli` - this will take some time
4. Copy `.env.example` to `.env`
5. Update using values from an IdP of your choice
6. Setup the SQLite database with `sqlx database setup`
7. Serve using `cargo run -- serve`

If you receive errors mentioning cranelift, consider removing that segment from
`sync/Cargo.toml`. The project is optimized for faster compiletime in dev, and
faster runtime at release. Release builds made with `--release` will compile
slower due to thjese runtime performance optimizations.

### Testing

Tests can be run with `cargo test`. Unit tests test specific small pieces of
code, and should live next to the source-code. These tests work well for
utilities. Integration tests are in the `tests` folder, and test it as if it is
an HTTP API.
