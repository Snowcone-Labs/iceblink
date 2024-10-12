use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct IceblinkInstanceMetadata {
    version: &'static str,
}

pub async fn index() -> (StatusCode, Json<IceblinkInstanceMetadata>) {
    (
        StatusCode::OK,
        Json(IceblinkInstanceMetadata {
            version: env!("CARGO_PKG_VERSION"),
        }),
    )
}
