FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev sqlite-dev
WORKDIR /iceblink
COPY . .
ENV DATABASE_URL=sqlite:iceblink.db
RUN cargo install sqlx-cli --no-default-features --features sqlite
RUN cargo sqlx database setup
RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache sqlite
RUN adduser --disabled-password --gecos "" --home "/home/iceblinker" --shell "/sbin/nologin" --uid 10001 iceblinker
WORKDIR /home/iceblinker
COPY --from=builder --chown=iceblinker:iceblinker /iceblink/target/release/iceblink-sync .
USER iceblinker:iceblinker
ENTRYPOINT [ "./iceblink-sync", "serve" ]
