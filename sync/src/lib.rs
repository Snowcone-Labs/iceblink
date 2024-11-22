pub mod auth;
pub mod cli;
pub mod models;
pub mod routes;
pub mod utils;

use axum::http::{Method, Request};
use axum::{middleware, Router};
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
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;

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

#[derive(OpenApi)]
#[openapi(
	tags(
		(name = "codes", description = "Code management endpoints"),
		(name = "user", description = "User endpoints"),
		(name = "misc", description = "Other endpoints")
	),
	servers(
		(url = "http://localhost:8085", description = "Local development server"),
		(url = "https://iceblink.snowflake.blue", description = "Production server")
	),
	info(
		title ="IceBlink Sync Server",
		contact(
			url="https://snowflake.blue",
			name="Snowflake-Software",
		),
		license(
			name="AGPLv3",
			identifier="AGPL-3.0-or-later"
		)
	)
)]
pub struct ApiDocumentation;

#[bon::builder]
pub fn configure_router(pool: &SqlitePool, opts: ServerOptions, openid: auth::OpenId) -> Router {
    let state = Arc::new(AppState {
        db: pool.clone(),
        settings: opts.clone(),
        openid,
    });

    // Note: Read bottom to top
    let (router, api) = OpenApiRouter::with_openapi(ApiDocumentation::openapi())
        .routes(routes!(
            routes::v1::codes::list_all_codes,
            routes::v1::codes::add_code
        ))
        .routes(routes!(
            routes::v1::codes::delete_code,
            routes::v1::codes::edit_code
        ))
        .routes(routes!(routes::v1::users::delete_account))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth::jwt_middleware,
        ))
        .routes(routes!(routes::v1::index::index))
        .routes(routes!(routes::v1::users::oauth))
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
        .split_for_parts();
    router
        .merge(SwaggerUi::new("/swagger").url("/openapi.json", api))
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
        .layer(TimeoutLayer::new(Duration::from_secs(2)))
}

pub async fn serve(opts: ServerOptions) {
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
    let openid = auth::OpenId::discover()
        .client_id(opts.clone().client_id)
        .client_secret(opts.clone().client_secret)
        .server(opts.clone().oauth_server)
        .call()
        .await
        .expect("Unable to setup OpenId authentication");

    info!("Configuring HTTP router");
    let routes = configure_router()
        .pool(&pool)
        .opts(opts.clone())
        .openid(openid)
        .call();

    info!("Starting HTTP server");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", opts.port))
        .await
        .unwrap();

    info!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, routes)
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
