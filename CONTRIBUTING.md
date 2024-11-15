# Contributing to IceBlink

## Sync

1. Move into `sync` folder
2. Install a recent version of Rust using `rustup`
3. Copy `.env.example` to `.env`
4. Update using values from a IdP of your choice
5. Serve using `cargo run -- serve`

### Testing

Tests can be run with `cargo test`. Unit tests test specific small pieces of
code, and should live next to the source-code. These tests work well for
utilities. Integration tests are in the `tests` folder, and test it as if it is
an HTTP API.
