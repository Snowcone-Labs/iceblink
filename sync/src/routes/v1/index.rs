use crate::AppState;
use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct IceblinkInstanceMetadata {
    version: &'static str,
    client_id: String,
    authorize: String,
    redirect_uri: String,
}

#[axum::debug_handler]
pub async fn index(
    State(data): State<Arc<AppState>>,
) -> (StatusCode, Json<IceblinkInstanceMetadata>) {
    (
        StatusCode::OK,
        Json(IceblinkInstanceMetadata {
            version: env!("CARGO_PKG_VERSION"),
            authorize: data.openid.authorization.clone(),
            client_id: data.openid.client_id.clone(),
            redirect_uri: data.settings.redirect_uri.clone(),
        }),
    )
}
