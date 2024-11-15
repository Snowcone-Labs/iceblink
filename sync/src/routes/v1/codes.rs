use crate::{
    models::{codes::Code, user::User},
    utils, AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde::Deserialize;
use std::sync::Arc;

use super::ApiError;

pub async fn list_all(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> Json<Vec<Code>> {
    Json(
        Code::get_many(&state.db, user.id)
            .await
            .expect("Unable to find codes owned by user"),
    )
}

#[derive(Deserialize)]
pub struct CodeAddPayload {
    pub content: String,
    pub display_name: String,
    pub website_url: Option<String>,
}

pub async fn add(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<CodeAddPayload>,
) -> Json<Code> {
    let code = Code {
        id: utils::generate_id(16),
        owner_id: user.id,
        content: payload.content,
        display_name: payload.display_name,
        website_url: payload.website_url,
        icon_url: None,
    };

    code.insert(&state.db).await.expect("Unable to insert code");
    Json(code)
}

pub async fn delete(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    Code::get(&state.db, id, user.id)
        .await?
        .delete(&state.db)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
