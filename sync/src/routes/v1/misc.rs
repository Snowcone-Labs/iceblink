use crate::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
pub struct IceblinkInstanceMetadata {
    version: String,
    client_id: String,
    authorize: String,
    redirect_uri: String,
}

#[utoipa::path(
	get,
	path = "/v1/",
	responses(
		(status = OK, description = "Successfully fetched instance metadata", body = IceblinkInstanceMetadata)
	),
	tag = "misc",
	security(())
)]
pub async fn instance_metadata(
    State(data): State<Arc<AppState>>,
) -> (StatusCode, Json<IceblinkInstanceMetadata>) {
    (
        StatusCode::OK,
        Json(IceblinkInstanceMetadata {
            version: env!("CARGO_PKG_VERSION").to_string(),
            authorize: data.openid.authorization.clone(),
            client_id: data.openid.client_id.clone(),
            redirect_uri: data.settings.redirect_uri.clone(),
        }),
    )
}

#[utoipa::path(
	get,
	path = "/v1/metrics",
	responses(
		(status = OK, description = "Successfully fetched prometheus-style metrics")
	),
	tag = "misc",
	security(())
)]
pub async fn metrics(State(data): State<Arc<AppState>>) -> impl IntoResponse {
    data.metrics.render()
}
