use crate::{
    auth,
    models::{self, user::User},
    server::AppState,
    utils,
};
use axum::{
    extract::{Query, State},
    http::HeaderMap,
};
use reqwest::{header, StatusCode};
use serde::Deserialize;
use std::sync::Arc;
use tracing::warn;

#[derive(Deserialize)]
pub struct OauthQueryParams {
    code: String,
}

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
