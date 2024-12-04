use axum::{body::Body, http::Method, http::Request};
use serde_json::json;
use sqlx::SqlitePool;
use tower::ServiceExt;

pub mod common;

#[sqlx::test(fixtures("users", "codes"))]
pub async fn list_codes_no_header(db: SqlitePool) {
    let app = common::testing_setup(&db).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/code")
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
            "errorKind": "MissingAuthentication"
        })
    );
}

#[sqlx::test(fixtures("users", "codes"))]
pub async fn list_codes_empty_header(db: SqlitePool) {
    let app = common::testing_setup(&db).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/code")
                .header("Authorization", "")
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
            "errorKind": "MissingAuthentication"
        })
    );
}

#[sqlx::test(fixtures("users", "codes"))]
pub async fn list_codes_empty_bearer(db: SqlitePool) {
    let app = common::testing_setup(&db).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/code")
                .header("Authorization", "Bearer ")
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
            "errorKind": "MissingAuthentication"
        })
    );
}

#[sqlx::test(fixtures("users", "codes"))]
pub async fn list_codes_garbage_bearer(db: SqlitePool) {
    let app = common::testing_setup(&db).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/code")
                .header("Authorization", "Bearer some funny garbage")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
    assert_eq!(
        common::convert_response(response).await,
        json!({
            "message": "The supplied authentication is invalid.",
            "errorKind": "InvalidAuthentication"
        })
    );
}

#[sqlx::test(fixtures("users", "codes"))]
pub async fn list_codes_invalid_signature(db: SqlitePool) {
    let app = common::testing_setup(&db).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/code")
                .header("Authorization", "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
    assert_eq!(
        common::convert_response(response).await,
        json!({
            "message": "The supplied authentication has an invalid signature. Try logging in again.",
            "errorKind": "InvalidJwtSignature"
        })
    );
}
