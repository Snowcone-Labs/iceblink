use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use common::AsExpected;
use iceblink_sync::models;
use sqlx::SqlitePool;
use tower::ServiceExt;

pub mod common;

#[sqlx::test(fixtures("users", "codes"))]
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
    assert_eq!(user1_delete.status(), StatusCode::NO_CONTENT);

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
    assert_eq!(user1_codes_after_deleted.status(), StatusCode::UNAUTHORIZED);

    // User2 still works as usual
    let u2 = common::list_codes_content(&app, a2.as_str()).await;
    assert_eq!(u2.len(), 1);
    for code in u2.iter() {
        assert!(code.is_as_expected())
    }

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
