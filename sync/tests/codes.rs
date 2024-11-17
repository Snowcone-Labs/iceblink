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
                "content": common::USER2_CODE1_CONTENT,
                "display_name": "Dummy INC",
                "icon_url": "https://dummy.com/favicon.ico",
                "id": common::USER2_CODE1_ID,
                "owner_id": common::USER2_ID,
                "website_url": "dummy.com"
            }
        ])
    );
}

#[sqlx::test(fixtures("users", "codes"))]
async fn add_codes(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, a2) = common::get_access_tokens(&db).await;

    // Add code
    let added = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri("/v1/codes")
                .header("Authorization", format!("Bearer {a1}"))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "content": "garbage",
                        "display_name": "Permafrost",
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(added.status(), StatusCode::OK);
    let added_res: models::codes::Code =
        serde_json::from_value(common::convert_response(added).await).unwrap();
    assert_eq!(added_res.content, "garbage");
    assert_eq!(added_res.display_name, "Permafrost");
    assert_eq!(added_res.icon_url, None);
    assert_eq!(added_res.website_url, None);
    assert_eq!(added_res.owner_id, common::USER1_ID);

    // Check that it was added to the list
    let user1_after = app
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

    assert_eq!(user1_after.status(), StatusCode::OK);
    assert_eq!(
        common::convert_response(user1_after).await,
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
            {
                "content": "garbage",
                "display_name": "Permafrost",
                "icon_url": null,
                "id": added_res.id,
                "owner_id": common::USER1_ID,
                "website_url": null
            },
        ])
    );

    // User 2 should not affected by the operation
    let user2_codes = app
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
    assert_eq!(user2_codes.status(), StatusCode::OK);
    assert_eq!(
        common::convert_response(user2_codes).await,
        json!([
            {
                "content": "djnaW1Pl2WjhWrU6",
                "display_name": "Dummy INC",
                "icon_url": "https://dummy.com/favicon.ico",
                "id": "fUJveqJaNpPhTUkR",
                "owner_id": common::USER2_ID,
                "website_url": "dummy.com"
            }
        ])
    );
}
