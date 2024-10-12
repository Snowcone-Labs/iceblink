use crate::{
    auth,
    models::{self},
    server::AppState,
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
    let exchange = state.openid.clone().exchange(code).await;
    let mut headers = HeaderMap::default();

    if let Ok(access_token) = exchange {
        let user_info = state.openid.clone().userinfo(access_token).await;

        if let Ok(user_data) = user_info {
            println!("Received user data! {:?}", user_data);

            let attemped_user_fetch =
                models::user::User::get_by_upstream_id(&state.db, user_data.id).await;

            if let Ok(user_fetch) = attemped_user_fetch {
                if let Some(user) = user_fetch {
                    let jwt = auth::create_jwt(&user, state.settings.jwt_secret.clone()).await;

                    headers.insert(header::SET_COOKIE, format!("jwt={jwt}").parse().unwrap());

                    (StatusCode::OK, headers)
                } else {
                    todo!("create user");
                }
            } else {
                warn!("Database error catched");
                (StatusCode::INTERNAL_SERVER_ERROR, headers)
            }
        } else {
            println!("Failed to get user data: {:?}", user_info.unwrap());
            (StatusCode::UNAUTHORIZED, headers)
        }
    } else {
        println!("Failed to exchange: {:?}", exchange.unwrap());
        (StatusCode::UNAUTHORIZED, headers)
    }
}
