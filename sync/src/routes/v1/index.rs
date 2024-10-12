use crate::server::AppState;
use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct IceblinkInstanceMetadata {
    version: &'static str,
    client_id: String,
    authorize: String,
}

pub async fn index(
    State(data): State<Arc<AppState>>,
) -> (StatusCode, Json<IceblinkInstanceMetadata>) {
    (
        StatusCode::OK,
        Json(IceblinkInstanceMetadata {
            version: env!("CARGO_PKG_VERSION"),
            authorize: data.settings.oauth.config.authorization_endpoint.clone(),
            client_id: data.settings.oauth.client_id.clone(),
        }),
    )
}
