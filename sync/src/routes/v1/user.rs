use crate::{
    auth,
    models::{self, user::User},
    utils, AppState,
};
use axum::{
    extract::{Query, State},
    http::HeaderMap,
    Extension,
};
use reqwest::{header, StatusCode};
use serde::Deserialize;
use std::sync::Arc;
use tracing::warn;
use utoipa::IntoParams;

use super::ApiError;

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
	)
)]
pub async fn oauth(
    State(state): State<Arc<AppState>>,
    query: Query<OauthQueryParams>,
) -> (StatusCode, HeaderMap) {
    let code = query.code.to_string();
    let mut headers = HeaderMap::default();

    let access_token = match state.openid.clone().exchange(code.clone()).await {
        Ok(token) => token,
        Err(e) => {
            warn!("Error occoured while exchanging code for token: {}", e);
            return (StatusCode::UNAUTHORIZED, headers.clone());
        }
    };

    let userinfo = match state.openid.clone().userinfo(access_token).await {
        Ok(userinfo) => userinfo,
        Err(e) => {
            warn!("Error occoured while fetching user info: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, headers.clone());
        }
    };

    let user_query =
        match models::user::User::get_by_upstream_id(&state.db, userinfo.clone().id).await {
            Ok(user_query) => user_query,
            Err(e) => {
                warn!("Database error catched: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, headers);
            }
        };

    if let Some(user) = user_query {
        let (_, cookie) = auth::create_jwt(&user, state.settings.jwt_secret.clone()).await;

        headers.insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

        (StatusCode::OK, headers)
    } else {
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
        user.insert(&state.db).await.unwrap();
        let (_, cookie) = auth::create_jwt(&user, state.settings.jwt_secret.clone()).await;

        headers.insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

        (StatusCode::OK, headers)
    }
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