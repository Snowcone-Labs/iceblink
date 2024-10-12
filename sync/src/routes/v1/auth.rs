use crate::{models::user, server::AppState};
use axum::{
    extract::{Query, State},
    http::HeaderMap,
};
use reqwest::{header, StatusCode};
use serde::Deserialize;
use std::sync::Arc;

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

    if let Ok(access_token) = exchange {
        let user_info = state.openid.clone().userinfo(access_token).await;

        if let Ok(user_data) = user_info {
            println!("Received user data! {:?}", user_data);
        } else {
            println!("Failed to get user data: {:?}", user_info.unwrap());
            return (StatusCode::UNAUTHORIZED, HeaderMap::default());
        }

        let mut headers = HeaderMap::new();
        // headers.insert(header::SET_COOKIE, format!("jwt={jwt}").parse().unwrap());

        (StatusCode::OK, headers)
    } else {
        println!("Failed to exchange: {:?}", exchange.unwrap());
        (StatusCode::UNAUTHORIZED, HeaderMap::default())
    }
}
