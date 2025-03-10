use super::{ApiError, JSON};
use crate::{
    auth,
    models::{self, codes::Code, user::User},
    utils, AppState,
};
use axum::{
    extract::{Query, State},
    http::HeaderMap,
    Extension,
};
use reqwest::{header, StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct OauthQueryParams {
    code: String,
}

#[utoipa::path(
	method(get),
	path = "/v1/oauth",
	tag = "user",
	responses(
		(status = OK, description = "Success")
	),
	params(
		OauthQueryParams
	),
	security(())
)]
pub async fn oauth(
    State(state): State<Arc<AppState>>,
    query: Query<OauthQueryParams>,
) -> Result<(StatusCode, HeaderMap), ApiError> {
    let code = query.code.to_string();
    let mut headers = HeaderMap::default();

    let access_token = state
        .openid
        .clone()
        .exchange(code.clone())
        .await
        .map_err(ApiError::OpenIdTokenExchangeFail)?;

    let userinfo = state
        .openid
        .clone()
        .userinfo(access_token)
        .await
        .map_err(ApiError::OpenIdUserinfoFail)?;

    let user_query = models::user::User::get_by_upstream_id(&state.db, userinfo.clone().id).await?;

    let user = match user_query {
        None => {
            let user = User {
                avatar_url: userinfo.clone().avatar,
                display_name: userinfo
                    .clone()
                    .display_name
                    .unwrap_or(userinfo.clone().username),
                id: utils::generate_id(16),
                upstream_userid: userinfo.clone().id,
                username: userinfo.clone().username,
            };
            user.insert(&state.db).await?;
            user
        }
        Some(user) => user,
    };

    let (_, cookie) = auth::create_jwt(&user, state.settings.jwt_secret.clone()).await;
    headers.insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok((StatusCode::OK, headers))
}

#[utoipa::path(
	method(delete),
	path = "/v1/user",
	tag = "user",
	responses(
		(status = NO_CONTENT, description = "Successfully deleted")
	),
)]
pub async fn delete_account(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> Result<StatusCode, ApiError> {
    user.delete(&state.db).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema, Clone)]
pub struct ChecksumResponse {
    pub checksum: String,
}

#[utoipa::path(
	get,
	path = "/v1/user/checksum",
	tag = "user",
	responses(
		(status = OK, description = "Successfully calculated checksum of user", body = ChecksumResponse)
	),
)]
pub async fn checksum(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> Result<JSON<ChecksumResponse>, ApiError> {
    let codes = Code::get_many(&state.db, user.clone().id).await?;

    Ok(JSON(ChecksumResponse {
        checksum: utils::checksum(codes, &user),
    }))
}
