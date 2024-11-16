use std::usize;

use axum::{response::Response, Router};
use iceblink_sync::{
    auth::{self, OpenId},
    configure_router, ServerOptions,
};
use sqlx::SqlitePool;

pub async fn testing_setup(pool: &SqlitePool) -> Router {
    configure_router()
        .pool(pool)
        .openid(OpenId {
            authorization: "N/A".into(),
            client_id: "N/A".into(),
            client_secret: "N/A".into(),
            token: "N/A".into(),
            userinfo: "N/A".into(),
        })
        .opts(ServerOptions {
            port: 8000,
            jwt_secret: "my jwt secret".into(),
            client_id: "N/A".into(),
            client_secret: "N/A".into(),
            oauth_server: "N/A".into(),
            redirect_uri: "N/A".into(),
        })
        .call()
}

pub async fn get_access_tokens(pool: &SqlitePool) -> (String, String) {
    let user1 = iceblink_sync::models::user::User::get_by_id(&pool, "k0d8WrkRjK6gkc3C".into())
        .await
        .unwrap()
        .unwrap();
    let user2 = iceblink_sync::models::user::User::get_by_id(&pool, "3Ck0d8WrkRjK6gkc".into())
        .await
        .unwrap()
        .unwrap();

    (
        auth::create_jwt(&user1, "my jwt secret".into()).await.0,
        auth::create_jwt(&user2, "my jwt secret".into()).await.0,
    )
}

pub async fn convert_response(response: Response) -> serde_json::Value {
    serde_json::from_str(
        String::from_utf8(
            axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap()
        .as_str(),
    )
    .unwrap()
}

// https://docs.rs/sqlx/latest/sqlx/attr.test.html
