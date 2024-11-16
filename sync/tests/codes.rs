use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use serde_json::json;
use sqlx::SqlitePool;
use tower::ServiceExt;

mod common;

#[sqlx::test(fixtures("users", "codes"))]
async fn list_own_codes(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, a2) = common::get_access_tokens(&db).await;

    let response_user1 = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/codes")
                .header("Authorization", format!("Bearer {a1}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response_user1.status(), StatusCode::OK);
    assert_eq!(
        common::convert_response(response_user1).await,
        json!([
            {
                "content": "GK6ZFMqk18fuWnCw",
                "display_name": "Google",
                "icon_url": null,
                "id": "Ckpt4eFi1pw9fxI3",
                "owner_id": "k0d8WrkRjK6gkc3C",
                "website_url": "google.com"
            },
            {
                "content": "XGDi8FlvZ5OGBoxG",
                "display_name": "google.com",
                "icon_url": null,
                "id": "DxLCqi4ZlHPD8YxA",
                "owner_id": "k0d8WrkRjK6gkc3C",
                "website_url": "google.com"
            },
        ])
    );

    let response_user2 = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/codes")
                .header("Authorization", format!("Bearer {a2}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response_user2.status(), StatusCode::OK);
    assert_eq!(
        common::convert_response(response_user2).await,
        json!([
            {
                "content": "djnaW1Pl2WjhWrU6",
                "display_name": "Dummy INC",
                "icon_url": "https://dummy.com/favicon.ico",
                "id": "fUJveqJaNpPhTUkR",
                "owner_id": "3Ck0d8WrkRjK6gkc",
                "website_url": "dummy.com"
            }
        ])
    );
}
