// https://docs.rs/sqlx/latest/sqlx/attr.test.html

use axum::{
    body::Body,
    http::{Method, Request},
    response::Response,
    Router,
};
use iceblink_sync::{
    auth::{self, OpenId},
    configure_router, models,
    routes::v1::users::ChecksumResponse,
    ServerOptions,
};
use sqlx::SqlitePool;
use std::usize;
use tower::ServiceExt;

pub const USER1_ID: &str = "k0d8WrkRjK6gkc3C";
pub const USER2_ID: &str = "3Ck0d8WrkRjK6gkc";
pub const USER1_CODE1_ID: &str = "Ckpt4eFi1pw9fxI3";
pub const USER1_CODE2_ID: &str = "DxLCqi4ZlHPD8YxA";
pub const USER2_CODE1_ID: &str = "fUJveqJaNpPhTUkR";
pub const USER1_CODE1_CONTENT: &str = "GK6ZFMqk18fuWnCw";
pub const USER1_CODE2_CONTENT: &str = "XGDi8FlvZ5OGBoxG";
pub const USER2_CODE1_CONTENT: &str = "djnaW1Pl2WjhWrU6";

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
            frontfacing: "N/A".into(),
        })
        .call()
}

pub async fn get_access_tokens(pool: &SqlitePool) -> (String, String) {
    let user1 = iceblink_sync::models::user::User::get_by_id(&pool, USER1_ID.into())
        .await
        .unwrap()
        .unwrap();
    let user2 = iceblink_sync::models::user::User::get_by_id(&pool, USER2_ID.into())
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

pub async fn list_codes(app: &Router, token: &str) -> Response {
    app.clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/code")
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
}

pub async fn list_codes_content(app: &Router, token: &str) -> Vec<models::codes::Code> {
    serde_json::from_value(convert_response(list_codes(app, token).await).await).unwrap()
}

pub async fn add_code(app: &Router, token: &str, payload: &serde_json::Value) -> Response {
    app.clone()
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri("/v1/code")
                .header("Authorization", format!("Bearer {token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap()
}

pub async fn delete_code(app: &Router, token: &str, id: &str) -> Response {
    app.clone()
        .oneshot(
            Request::builder()
                .method(Method::DELETE)
                .uri(format!("/v1/code/{id}"))
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
}

pub async fn edit_code(
    app: &Router,
    token: &str,
    id: &str,
    payload: &serde_json::Value,
) -> Response {
    app.clone()
        .oneshot(
            Request::builder()
                .method(Method::PATCH)
                .uri(format!("/v1/code/{id}"))
                .header("Authorization", format!("Bearer {token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap()
}

pub async fn user_checksum(app: &Router, token: &str) -> String {
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/user/checksum")
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let parsed: ChecksumResponse = serde_json::from_value(convert_response(res).await).unwrap();
    parsed.checksum
}

pub trait AsExpected {
    fn is_as_expected(&self) -> bool;
}

impl AsExpected for models::codes::Code {
    fn is_as_expected(&self) -> bool {
        match self.id.as_str() {
            USER1_CODE1_ID => {
                self.content == USER1_CODE1_CONTENT
                    && self.display_name == "Google"
                    && self.icon_url == None
                    && self.owner_id == USER1_ID
                    && self.website_url == Some("google.com".to_string())
            }
            USER1_CODE2_ID => {
                self.content == USER1_CODE2_CONTENT
                    && self.display_name == "google.com"
                    && self.icon_url == None
                    && self.owner_id == USER1_ID
                    && self.website_url == Some("google.com".to_string())
            }
            USER2_CODE1_ID => {
                self.content == USER2_CODE1_CONTENT
                    && self.display_name == "Dummy INC"
                    && self.icon_url == Some("https://dummy.com/favicon.ico".to_string())
                    && self.owner_id == USER2_ID
                    && self.website_url == Some("dummy.com".to_string())
            }
            _ => false,
        }
    }
}
