use super::ApiError;
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
use utoipa::ToSchema;

#[utoipa::path(
	get,
	path = "/v1/code",
	responses(
		(status = OK, description = "Successfully fetches codes", body = Vec<Code>)
	),
	tag = "codes",
)]
pub async fn list_all_codes(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> Json<Vec<Code>> {
    Json(
        Code::get_many(&state.db, user.id)
            .await
            .expect("Unable to find codes owned by user"),
    )
}

#[derive(Deserialize, ToSchema)]
pub struct CodeAddPayload {
    pub content: String,
    pub display_name: String,
    pub website_url: Option<String>,
}

#[utoipa::path(
	method(put),
	path = "/v1/code",
	responses(
		(status = OK, description = "Succesfully created code. Response contains contents of the new code", body = Code)
	),
	request_body = CodeAddPayload,
	tag = "codes"
)]
pub async fn add_code(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<CodeAddPayload>,
) -> Result<Json<Code>, ApiError> {
    let code = Code {
        id: utils::generate_id(16),
        owner_id: user.id,
        content: payload.content,
        display_name: payload.display_name,
        website_url: payload.website_url,
        icon_url: None,
    };

    code.insert(&state.db).await?;
    Ok(Json(code))
}

#[derive(Deserialize, ToSchema)]
pub struct CodeEditPayload {
    pub content: Option<String>,
    pub display_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub website_url: Option<Option<String>>,
}

#[utoipa::path(
	method(patch),
	path = "/v1/code/{id}",
	tag = "codes",
	params(
		("id", description = "Id of the code to edit")
	),
	request_body = CodeEditPayload,
	responses(
		(status = OK, description = "Success", body = Vec<Code>)
	),
)]
pub async fn edit_code(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    Json(payload): Json<CodeEditPayload>,
) -> Result<Json<Code>, ApiError> {
    Ok(Json(
        Code::get(&state.db, id, user.id)
            .await?
            .ok_or(ApiError::NotFound)?
            .edit()
            .pool(&state.db)
            .maybe_content(payload.content)
            .maybe_display_name(payload.display_name)
            .maybe_website_url(payload.website_url)
            .call()
            .await?
            .clone(),
    ))
}

#[utoipa::path(
	method(delete),
	path = "/v1/code/{id}",
	tag = "codes",
	responses(
		(status = NO_CONTENT, description = "Deleted")
	),
	params(
		("id", description = "Id of code to delete")
	)
)]
pub async fn delete_code(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    Code::get(&state.db, id, user.id)
        .await?
        .ok_or(ApiError::NotFound)?
        .delete(&state.db)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
