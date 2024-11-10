use crate::{auth, routes};
use axum::http::{Method, Request};
use axum::middleware;
use axum::routing::{delete, get, put};
use memory_serve::{load_assets, MemoryServe};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::{info, Span};

#[derive(Clone)]
pub struct ServerOptions {
    pub port: u32,
    pub jwt_secret: String,
    pub client_id: String,
    pub client_secret: String,
    pub oauth_server: String,
    pub redirect_uri: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub settings: ServerOptions,
    pub openid: auth::OpenId,
}

pub async fn create_server(opts: ServerOptions) {
    info!("Connecting to SQLite: iceblink.db");
    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename("iceblink.db")
            .create_if_missing(true),
    )
    .await
    .expect("Unable to connect with SQLite");

    info!("Running SQL migrations");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Unable to run database migrations");

    info!("Discovering OpenId configuration");
    let openid = auth::OpenId::new(
        opts.clone().client_id,
        opts.clone().client_secret,
        opts.clone().oauth_server,
    )
    .await
    .expect("Unable to setup OpenId authentication");

    info!("Configuring HTTP server");
    let state = Arc::new(AppState {
        db: pool,
        settings: opts.clone(),
        openid,
    });

    // Note: Read bottom to top
    let app = axum::Router::new()
        .route("/v1/codes", get(routes::v1::codes::list_all))
        .route("/v1/codes", put(routes::v1::codes::add))
        .route("/v1/code/:uuid", delete(routes::v1::codes::delete))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth::jwt_middleware,
        ))
        .route("/v1/", get(routes::v1::index::index))
        .route("/v1/oauth", get(routes::v1::auth::oauth))
        .with_state(state)
        .nest_service(
            "/",
            MemoryServe::new(load_assets!("./src/static"))
                .index_file(Some("/landing.html"))
                .html_cache_control(memory_serve::CacheControl::Long)
                .enable_clean_url(true)
                .enable_brotli(true)
                .enable_gzip(true)
                .into_router(),
        )
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET])
                .allow_origin(tower_http::cors::Any),
        )
        .layer(CompressionLayer::new())
        .layer(
            TraceLayer::new_for_http().on_request(|request: &Request<_>, _span: &Span| {
                info!("{} {}", request.method(), request.uri().path())
            }),
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
