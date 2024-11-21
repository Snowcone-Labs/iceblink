use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use iceblink_sync::models;
use serde_json::json;
use sqlx::SqlitePool;
use tower::ServiceExt;

pub mod common;

#[sqlx::test(fixtures("users", "codes"))]
async fn delete_account(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, a2) = common::get_access_tokens(&db).await;

    let user1_still_works = app
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
    assert_eq!(user1_still_works.status(), StatusCode::OK);
    assert_eq!(
        common::convert_response(user1_still_works).await,
        json!([
            {
                "content": common::USER1_CODE1_CONTENT,
                "display_name": "Google",
                "icon_url": null,
                "id": common::USER1_CODE1_ID,
                "owner_id": common::USER1_ID,
                "website_url": "google.com"
            },
            {
                "content": common::USER1_CODE2_CONTENT,
                "display_name": "google.com",
                "icon_url": null,
                "id": common::USER1_CODE2_ID,
                "owner_id": common::USER1_ID,
                "website_url": "google.com"
            },
        ])
    );

    let user1_delete = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::DELETE)
                .uri("/v1/user")
                .header("Authorization", format!("Bearer {a1}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(user1_delete.status(), StatusCode::NO_CONTENT);

    let user1_no_auth = app
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
    assert_eq!(user1_no_auth.status(), StatusCode::UNAUTHORIZED);

    // User 2 should not be affected by User 1 deleteing their account.
    let user2_still_works = app
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
    assert_eq!(user2_still_works.status(), StatusCode::OK);
    assert_eq!(
        common::convert_response(user2_still_works).await,
        json!([
            {
                "content": common::USER2_CODE1_CONTENT,
                "display_name": "Dummy INC",
                "icon_url": "https://dummy.com/favicon.ico",
                "id": common::USER2_CODE1_ID,
                "owner_id": common::USER2_ID,
                "website_url": "dummy.com"
            }
        ])
    );

    // Check that the user is actually deleted from database.
    assert!(models::user::User::get_by_id(&db, common::USER1_ID.into())
        .await
        .unwrap()
        .is_none());

    // Check that the user codes are deleted
    assert!(
        models::codes::Code::get(&db, common::USER1_CODE1_ID.into(), common::USER1_ID.into())
            .await
            .unwrap()
            .is_none(),
    );
}
