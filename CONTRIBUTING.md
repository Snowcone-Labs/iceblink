# Contributing to IceBlink

## Sync

1. Move into `sync` folder
2. Install a recent version of Rust using `rustup`
3. Install the sqlx CLI using `cargo install sqlx-cli` - this will take some time
4. Copy `.env.example` to `.env`
5. Update using values from an IdP of your choice
6. Setup the SQLite database with `sqlx database setup`
7. Serve using `cargo run -- serve`

### Testing

Tests can be run with `cargo test`. Unit tests test specific small pieces of
code, and should live next to the source-code. These tests work well for
utilities. Integration tests are in the `tests` folder, and test it as if it is
an HTTP API.
