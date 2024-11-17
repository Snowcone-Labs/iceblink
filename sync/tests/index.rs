use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use serde_json::json;
use sqlx::SqlitePool;
use tower::ServiceExt;

pub mod common;

#[sqlx::test]
async fn index(db: SqlitePool) {
    let app = common::testing_setup(&db).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        common::convert_response(response).await,
        json!({
            "version": env!("CARGO_PKG_VERSION"),
            "authorize": "N/A",
            "client_id": "N/A",
            "redirect_uri": "N/A",
        })
    );
}
