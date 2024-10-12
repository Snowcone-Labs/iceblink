use axum::http::Method;
use axum::routing::{delete, get, put};
use memory_serve::{load_assets, MemoryServe};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tracing::info;

use crate::routes;

pub struct ServerOptions {
    pub port: u32,
}

pub struct AppState {
    pub db: SqlitePool,
}

pub async fn create_server(opts: ServerOptions) {
    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename("iceblink.db")
            .create_if_missing(true),
    )
    .await
    .expect("Unable to connect with SQLite");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Unable to run database migrations");

    let app = axum::Router::new()
        .route("/v1/", get(routes::v1::index::index))
        .route("/v1/oauth", get(routes::v1::auth::oauth))
        .route("/v1/codes", get(routes::v1::codes::list_all))
        .route("/v1/codes", put(routes::v1::codes::add))
        .route("/v1/code/:uuid", delete(routes::v1::codes::delete))
        .with_state(Arc::new(AppState { db: pool }))
        .nest_service(
            "/",
            MemoryServe::new(load_assets!("static"))
                .index_file(Some("/landing.html"))
                .html_cache_control(memory_serve::CacheControl::Long)
                .enable_clean_url(true)
                .enable_brotli(true)
                .enable_gzip(true)
                .into_router(),
        )
        .layer(
            ServiceBuilder::new().layer(
                CorsLayer::new()
                    .allow_methods([Method::GET])
                    .allow_origin(tower_http::cors::Any),
            ),
        )
        .layer(TimeoutLayer::new(Duration::from_secs(2)));

    info!("Starting HTTP server");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", opts.port))
        .await
        .unwrap();

    info!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Exit imminent")
}
