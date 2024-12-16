use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use common::AsExpected;
use googletest::prelude::*;
use iceblink_sync::models;
use serde_json::json;
use sqlx::SqlitePool;
use tower::ServiceExt;

pub mod common;

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn delete_account(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, a2) = common::get_access_tokens(&db).await;

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
    assert_that!(user1_delete.status(), eq(StatusCode::NO_CONTENT));

    let user1_codes_after_deleted = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/code")
                .header("Authorization", format!("Bearer {a1}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_that!(
        user1_codes_after_deleted.status(),
        eq(StatusCode::UNAUTHORIZED)
    );

    // User2 still works as usual
    let u2 = common::list_codes_content(&app, a2.as_str()).await;
    assert_that!(u2.len(), eq(1));
    for code in u2.iter() {
        assert!(code.is_as_expected())
    }

    // Check that the user is actually deleted from database.
    assert_that!(
        models::user::User::get_by_id(&db, common::USER1_ID.into())
            .await
            .unwrap(),
        none()
    );

    // Check that the user codes are deleted
    assert_that!(
        models::codes::Code::get(&db, common::USER1_CODE1_ID.into(), common::USER1_ID.into())
            .await
            .unwrap(),
        none()
    );
}

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn checksum_two_requests_equal(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, _) = common::get_access_tokens(&db).await;

    let checksum1 = common::user_checksum(&app, a1.as_str()).await;
    let checksum2 = common::user_checksum(&app, a1.as_str()).await;
    assert_that!(checksum1, eq(&checksum2));
}

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn checksum_changes_code_name(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, _) = common::get_access_tokens(&db).await;

    let checksum1 = common::user_checksum(&app, a1.as_str()).await;
    common::edit_code(
        &app,
        &a1,
        common::USER1_CODE1_ID,
        &json!({
            "display_name": "not sure honestly"
        }),
    )
    .await;
    let checksum2 = common::user_checksum(&app, a1.as_str()).await;
    assert_that!(checksum1, not(eq(&checksum2)));
}

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn checksum_changes_code_content(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, _) = common::get_access_tokens(&db).await;

    let checksum1 = common::user_checksum(&app, a1.as_str()).await;
    common::edit_code(
        &app,
        &a1,
        common::USER1_CODE1_ID,
        &json!({
            "content": "not sure honestly"
        }),
    )
    .await;
    let checksum2 = common::user_checksum(&app, a1.as_str()).await;
    assert_that!(checksum1, not(eq(&checksum2)));
}

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn checksum_equal_edit_and_revert_content(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, _) = common::get_access_tokens(&db).await;

    let checksum1 = common::user_checksum(&app, a1.as_str()).await;

    common::edit_code(
        &app,
        &a1,
        common::USER1_CODE1_ID,
        &json!({
            "content": "not sure honestly"
        }),
    )
    .await;
    common::edit_code(
        &app,
        &a1,
        common::USER1_CODE1_ID,
        &json!({
            "content": common::USER1_CODE1_CONTENT
        }),
    )
    .await;

    let checksum2 = common::user_checksum(&app, a1.as_str()).await;
    assert_that!(checksum1, eq(&checksum2));
}

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn checksum_changes_code_add(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, _) = common::get_access_tokens(&db).await;

    let checksum1 = common::user_checksum(&app, a1.as_str()).await;
    common::add_code(
        &app,
        &a1,
        &json!({
            "content": "garbage",
            "display_name": "Permafrost",
        }),
    )
    .await;
    let checksum2 = common::user_checksum(&app, a1.as_str()).await;
    assert_that!(checksum1, not(eq(&checksum2)));
}

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn checksum_changes_code_delete(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, _) = common::get_access_tokens(&db).await;

    let checksum1 = common::user_checksum(&app, a1.as_str()).await;
    common::delete_code(&app, &a1, common::USER1_CODE1_ID).await;
    let checksum2 = common::user_checksum(&app, a1.as_str()).await;

    assert_that!(checksum1, not(eq(&checksum2)));
}
