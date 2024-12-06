# Iceblink

TOTP authentication app by Snowflake-Software.

> [!IMPORTANT]
> The app is WIP, and not in a functioning state.

## App

The app uses Expo and React Native.

## Sync Server

The sync/backup server is written in Rust. It uses Axum for routing, and sqlx
with SQLite for database. All data is encrypted on the client. Configuration
options can be passed in the following ways:

- Flags
- Environment variables
- .env

The database manages itself, and you do not have to run migrations, nor create
the file.
