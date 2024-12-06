use std::collections::HashMap;

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use chrono::{DateTime, Utc};
use serde_json::json;
use sqlx::SqlitePool;
use tower::ServiceExt;

pub mod common;

#[sqlx::test]
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

#[sqlx::test]
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

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers().get("Content-Type").unwrap(), "text/html");
    assert_eq!(
        response.headers().get("Cache-Control").unwrap(),
        "max-age=31536000, immutable"
    );
}

#[sqlx::test]
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

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("Content-Type").unwrap(),
        "text/plain"
    );
}

#[sqlx::test]
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
    assert!(expiry > Utc::now())
}

#[sqlx::test]
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

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers().get("Content-Type").unwrap(), "text/html");
}

#[sqlx::test]
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

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(response.headers().get("Location").unwrap(), "/swagger/");
}

#[sqlx::test]
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

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("Content-Type").unwrap(),
        "application/json"
    );

    // check that it can parse
    common::convert_response(response).await;
}

#[sqlx::test]
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

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get("Access-Control-Allow-Origin")
            .unwrap(),
        "N/A"
    );
    assert_eq!(
        response
            .headers()
            .get("Access-Control-Allow-Headers")
            .unwrap(),
        "authorization,content-type"
    );
    assert_eq!(
        response
            .headers()
            .get("Access-Control-Allow-Credentials")
            .unwrap(),
        "true"
    );
}

#[sqlx::test]
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

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("Content-Type").unwrap(),
        "text/plain; charset=utf-8"
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

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("Content-Type").unwrap(),
        "text/plain; charset=utf-8"
    );

    let res = common::convert_response_str(response).await;
    let mut has_found_line = false;
    for line in res.lines() {
        if !line.starts_with("http_requests_total") {
            continue;
        }

        has_found_line = true;

        assert_eq!(
            line,
            "http_requests_total{method=\"GET\",path=\"/v1/metrics\",status=\"200\"} 1"
        )
    }
    assert!(has_found_line);
}
