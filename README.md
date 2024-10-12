# Iceblink

TOTP authenticator app by Snowflake-Software.

## Sync Server

The sync/backup server is written in Rust. It uses Axum for routing, and sqlx
with sqlite for database. All data is encrypted on the client. Configuration
options can be passed in the following ways:

- Flags
- Environment variables
- .env
