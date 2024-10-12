use axum::http::Method;
use axum::routing::get;
use memory_serve::{load_assets, MemoryServe};
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

pub async fn create_server(opts: ServerOptions) {
    let app = axum::Router::new()
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
        .layer(TimeoutLayer::new(Duration::from_secs(2)))
        .route("/v1/", get(routes::v1::index::index));

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
