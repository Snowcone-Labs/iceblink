use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use chrono::{DateTime, Utc};
use googletest::prelude::*;
use iceblink_sync::models;
use serde_json::json;
use sqlx::SqlitePool;
use std::collections::HashMap;
use tower::ServiceExt;

pub mod common;

#[sqlx::test]
#[gtest]
async fn api_metadata(db: SqlitePool) {
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

    assert_that!(response.status(), eq(StatusCode::OK));

    let converted = common::convert_response(response).await;
    assert_that!(
        converted,
        eq(&json!({
            "version": env!("CARGO_PKG_VERSION"),
            "authorize": "N/A",
            "client_id": "N/A",
            "redirect_uri": "N/A",
        }))
    );

    assert_that!(
        converted.get("version").unwrap().as_str().unwrap(),
        matches_regex(r"^(\d+\.)?(\d+\.)?(\*|\d+)$")
    );
}

#[sqlx::test]
#[gtest]
async fn landing_page(db: SqlitePool) {
    let app = common::testing_setup(&db).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_that!(response.status(), eq(StatusCode::OK));
    assert_that!(
        response.headers().get("Content-Type").unwrap(),
        eq("text/html")
    );
    assert_that!(
        response.headers().get("Cache-Control").unwrap(),
        eq("max-age=31536000, immutable")
    );
}

#[sqlx::test]
#[gtest]
async fn security_policy_serves(db: SqlitePool) {
    let app = common::testing_setup(&db).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/.well-known/security.txt")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_that!(response.status(), eq(StatusCode::OK));
    assert_that!(
        response.headers().get("Content-Type").unwrap(),
        eq("text/plain")
    );
}

#[sqlx::test]
#[gtest]
async fn security_policy_expiry(db: SqlitePool) {
    let app = common::testing_setup(&db).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/.well-known/security.txt")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let res = common::convert_response_str(response).await;
    let mut entries = HashMap::new();

    for line in res.lines() {
        let (key, value) = line.split_once(": ").unwrap();
        entries.insert(key, value);
    }

    let expiry_str = entries.get("Expires").unwrap();
    let expiry = expiry_str.parse::<DateTime<Utc>>().unwrap();
    assert_that!(expiry, gt(Utc::now()));
}

#[sqlx::test]
#[gtest]
async fn swagger(db: SqlitePool) {
    let app = common::testing_setup(&db).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/swagger/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_that!(response.status(), eq(StatusCode::OK));
    assert_that!(
        response.headers().get("Content-Type").unwrap(),
        eq("text/html")
    );
}

#[sqlx::test]
#[gtest]
async fn swagger_redirect(db: SqlitePool) {
    let app = common::testing_setup(&db).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/swagger")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_that!(response.status(), eq(StatusCode::SEE_OTHER));
    assert_that!(response.headers().get("Location").unwrap(), eq("/swagger/"));
}

#[sqlx::test]
#[gtest]
async fn openapi_spec(db: SqlitePool) {
    let app = common::testing_setup(&db).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/openapi.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_that!(response.status(), eq(StatusCode::OK));
    assert_that!(
        response.headers().get("Content-Type").unwrap(),
        eq("application/json")
    );

    // check that it can parse
    common::convert_response(response).await;
}

#[sqlx::test]
#[gtest]
async fn cors_headers(db: SqlitePool) {
    let app = common::testing_setup(&db).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::OPTIONS)
                .uri("/v1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_that!(response.status(), eq(StatusCode::OK));
    assert_that!(
        response
            .headers()
            .get("Access-Control-Allow-Origin")
            .unwrap(),
        eq("N/A")
    );
    assert_that!(
        response
            .headers()
            .get("Access-Control-Allow-Headers")
            .unwrap(),
        eq("authorization,content-type")
    );
    assert_that!(
        response
            .headers()
            .get("Access-Control-Allow-Credentials")
            .unwrap(),
        eq("true")
    );
}

#[sqlx::test]
#[gtest]
async fn export_prometheus_metrics(db: SqlitePool) {
    let app = common::testing_setup(&db).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_that!(response.status(), eq(StatusCode::OK));
    assert_that!(
        response.headers().get("Content-Type").unwrap(),
        eq("text/plain; charset=utf-8")
    );

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/v1/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_that!(response.status(), eq(StatusCode::OK));
    assert_that!(
        response.headers().get("Content-Type").unwrap(),
        eq("text/plain; charset=utf-8")
    );

    let res = common::convert_response_str(response).await;
    let mut has_found_line = false;
    for line in res.lines() {
        if !line.starts_with("http_requests_total") {
            continue;
        }

        has_found_line = true;

        assert_that!(
            line,
            eq("http_requests_total{method=\"GET\",path=\"/v1/metrics\",status=\"200\"} 1")
        )
    }
    assert_that!(has_found_line, is_true());
}

#[test]
fn common_code_is_expected_user1_code2() {
    assert_that!(
        common::matchers::code_is_expected(
            crate::common::USER1_ID,
            &models::codes::Code {
                id: "DxLCqi4ZlHPD8YxA".into(),
                owner_id: "k0d8WrkRjK6gkc3C".into(),
                content: "XGDi8FlvZ5OGBoxG".into(),
                display_name: "google.com".into(),
                icon_url: None,
                website_url: Some("google.com".into()),
            }
        ),
        is_true()
    );
}

#[test]
fn common_code_is_expected_user2_code1() {
    assert_that!(
        common::matchers::code_is_expected(
            crate::common::USER2_ID,
            &models::codes::Code {
                id: "fUJveqJaNpPhTUkR".into(),
                owner_id: "3Ck0d8WrkRjK6gkc".into(),
                content: "djnaW1Pl2WjhWrU6".into(),
                display_name: "Dummy INC".into(),
                icon_url: Some("https://dummy.com/favicon.ico".into()),
                website_url: Some("dummy.com".into()),
            }
        ),
        is_true()
    );
}

#[test]
fn common_code_is_expected_wrong_user() {
    assert_that!(
        common::matchers::code_is_expected(
            crate::common::USER1_ID,
            &models::codes::Code {
                id: "fUJveqJaNpPhTUkR".into(),
                owner_id: "3Ck0d8WrkRjK6gkc".into(),
                content: "djnaW1Pl2WjhWrU6".into(),
                display_name: "Dummy INC".into(),
                icon_url: Some("https://dummy.com/favicon.ico".into()),
                website_url: Some("dummy.com".into()),
            }
        ),
        is_false()
    );
}
