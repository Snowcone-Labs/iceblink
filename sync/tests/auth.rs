use axum::{body::Body, http::Method, http::Request};
use serde_json::json;
use sqlx::SqlitePool;
use tower::ServiceExt;

pub mod common;

#[sqlx::test(fixtures("users", "codes"))]
pub fn list_codes_no_header(db: SqlitePool) {
    let app = common::testing_setup(&db).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/codes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
    assert_eq!(
        common::convert_response(response).await,
        json!({
            "message": "Missing authentication. Supply a JWT in the `iceblink_jwt` cookie, or use a bearer in the `Authorization` header.",
            "type": "MissingAuthentication"
        })
    );
}
