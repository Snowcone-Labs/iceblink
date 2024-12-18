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
async fn list_own_codes(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, a2) = common::get_access_tokens(&db).await;

    let u1 = common::list_codes_content(&app, a1.as_str()).await;
    assert_that!(u1, common::matchers::code_fixture_default());

    let u2 = common::list_codes_content(&app, a2.as_str()).await;
    assert_that!(u2, common::matchers::code_fixture_default());
}

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn add_code(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, a2) = common::get_access_tokens(&db).await;

    // Add code
    let added = common::add_code(
        &app,
        &a1,
        &json!({
            "content": "garbage",
            "display_name": "Permafrost",
        }),
    )
    .await;

    assert_that!(added.status(), eq(StatusCode::OK));
    let added_res: models::codes::Code =
        serde_json::from_value(common::convert_response(added).await).unwrap();

    expect_that!(added_res.content, eq("garbage"));
    expect_that!(added_res.display_name, eq("Permafrost"));
    expect_that!(added_res.icon_url, none());
    expect_that!(added_res.website_url, none());
    expect_that!(added_res.owner_id, eq(common::USER1_ID));
    expect_that!(added_res.id.len(), eq(16));

    // Check that it was added to the list
    let listing_request = common::list_codes_content(&app, a1.as_str()).await;
    assert_that!(listing_request.len(), eq(3));

    for code in listing_request {
        if code.id == common::USER1_CODE1_ID || code.id == common::USER1_CODE2_ID {
            assert!(code.is_as_expected());
        } else {
            expect_that!(code.website_url, none());
            expect_that!(code.icon_url, none());
            expect_that!(code.content, eq("garbage"));
            expect_that!(code.owner_id, eq(common::USER1_ID));
            expect_that!(code.display_name, eq("Permafrost"));
        }
    }

    // User 2 should not be affected by the operation
    let u2 = common::list_codes_content(&app, a2.as_str()).await;
    assert_that!(u2, common::matchers::code_fixture_default());
}

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn add_code_no_content(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, _) = common::get_access_tokens(&db).await;

    // Add code
    let added = common::add_code(
        &app,
        &a1,
        &json!({
            "display_name": "Permafrost",
        }),
    )
    .await;

    assert_that!(added.status(), eq(StatusCode::BAD_REQUEST));
    assert_that!(
        common::convert_response(added).await,
        eq(&json!({
            "errorKind": "JsonDataError",
            "message": "Unable to process JSON. Are you missing a field? Tip: Check with the swagger documentation at /swagger!",
        }))
    );

    // Check that it was not added to the list
    let listing_request = common::list_codes_content(&app, a1.as_str()).await;
    assert_that!(listing_request, common::matchers::code_fixture_default());
}

//
// Code edit
//

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn edit_code_remove_website(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, _) = common::get_access_tokens(&db).await;

    let edit_request = common::edit_code(
        &app,
        a1.as_str(),
        common::USER1_CODE2_ID,
        &json!({
            "website_url": null
        }),
    )
    .await;

    // Returns updated code
    assert_that!(edit_request.status(), eq(StatusCode::OK));
    assert_that!(
        common::convert_response(edit_request).await,
        eq(&json!({
            "content": common::USER1_CODE2_CONTENT,
            "id": common::USER1_CODE2_ID,
            "owner_id": common::USER1_ID,
            "display_name": "google.com",
            "icon_url": null,
            "website_url": null
        }))
    );

    // The code is editted in the listing
    let listing_request = common::list_codes_content(&app, a1.as_str()).await;
    assert_that!(listing_request.len(), eq(2));

    let unmodified_code = listing_request
        .iter()
        .find(|v| v.id == common::USER1_CODE1_ID)
        .unwrap();
    assert!(unmodified_code.is_as_expected());

    let modified_code = listing_request
        .iter()
        .find(|v| v.id == common::USER1_CODE2_ID)
        .unwrap();
    expect_that!(modified_code.id, eq(common::USER1_CODE2_ID));
    expect_that!(modified_code.website_url, none());
    expect_that!(modified_code.icon_url, none());
    expect_that!(modified_code.content, eq(common::USER1_CODE2_CONTENT));
    expect_that!(modified_code.owner_id, eq(common::USER1_ID));
    expect_that!(modified_code.display_name, eq("google.com"));
}

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn edit_code_update_website_removes_icon(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (_, a2) = common::get_access_tokens(&db).await;

    let edit_request = common::edit_code(
        &app,
        a2.as_str(),
        common::USER2_CODE1_ID,
        &json!({
            "website_url": "example.com"
        }),
    )
    .await;

    assert_that!(edit_request.status(), eq(StatusCode::OK));
    assert_that!(
        common::convert_response(edit_request).await,
        eq(&json!({
            "content": common::USER2_CODE1_CONTENT,
            "id": common::USER2_CODE1_ID,
            "owner_id": common::USER2_ID,
            "display_name": "Dummy INC",
            "icon_url": null,
            "website_url": "example.com"
        }))
    );

    // The code is editted in the listing
    let listing_request = common::list_codes_content(&app, a2.as_str()).await;
    assert_that!(listing_request.len(), eq(1));
    let code = listing_request.get(0).unwrap();
    expect_that!(code.id, eq(common::USER2_CODE1_ID));
    expect_that!(code.website_url, some(eq("example.com")));
    expect_that!(code.icon_url, none());
    expect_that!(code.content, eq(common::USER2_CODE1_CONTENT));
    expect_that!(code.owner_id, eq(common::USER2_ID));
    expect_that!(code.display_name, eq("Dummy INC"));
}

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn edit_code_content_and_display_name(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, _) = common::get_access_tokens(&db).await;

    let edit_request = common::edit_code(
        &app,
        a1.as_str(),
        common::USER1_CODE2_ID,
        &json!({
            "content": "yippie",
            "display_name": "Modrinth"
        }),
    )
    .await;

    // Returns updated code
    assert_that!(edit_request.status(), eq(StatusCode::OK));
    assert_that!(
        common::convert_response(edit_request).await,
        eq(&json!({
            "content": "yippie",
            "id": common::USER1_CODE2_ID,
            "owner_id": common::USER1_ID,
            "display_name": "Modrinth",
            "icon_url": null,
            "website_url": "google.com"
        }))
    );

    // The code is editted in the listing
    let listing_request = common::list_codes_content(&app, a1.as_str()).await;
    assert_that!(listing_request.len(), eq(2));
    let unmodified_code = listing_request
        .iter()
        .find(|v| v.id == common::USER1_CODE1_ID)
        .unwrap();
    assert!(unmodified_code.is_as_expected());

    let modified_code = listing_request
        .iter()
        .find(|v| v.id == common::USER1_CODE2_ID)
        .unwrap();
    expect_that!(modified_code.id, eq(common::USER1_CODE2_ID));
    expect_that!(modified_code.website_url, some(eq("google.com")));
    expect_that!(modified_code.icon_url, none());
    expect_that!(modified_code.content, eq("yippie"));
    expect_that!(modified_code.owner_id, eq(common::USER1_ID));
    expect_that!(modified_code.display_name, eq("Modrinth"));
}

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn edit_code_not_found(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, _) = common::get_access_tokens(&db).await;

    let edit_request = common::edit_code(
        &app,
        a1.as_str(),
        "gibberish",
        &json!({
            "website_url": "example.com"
        }),
    )
    .await;

    assert_that!(edit_request.status(), eq(StatusCode::NOT_FOUND));
    assert_that!(
        common::convert_response(edit_request).await,
        eq(&json!({
            "message": "Resource not found.",
            "errorKind": "NotFound"
        }))
    );
}

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn edit_code_other_user(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, a2) = common::get_access_tokens(&db).await;

    let edit_request = common::edit_code(
        &app,
        a1.as_str(),
        common::USER2_CODE1_ID,
        &json!({
            "display_name": "Hacked."
        }),
    )
    .await;

    assert_that!(edit_request.status(), eq(StatusCode::NOT_FOUND));
    assert_that!(
        common::convert_response(edit_request).await,
        eq(&json!({
            "message": "Resource not found.",
            "errorKind": "NotFound"
        }))
    );

    // Check that it did indeed not happen
    let u2 = common::list_codes_content(&app, a2.as_str()).await;
    assert_that!(u2, common::matchers::code_fixture_default());
}

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn edit_code_other_user_no_auth(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (_, a2) = common::get_access_tokens(&db).await;

    let edit_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::PATCH)
                .uri(format!("/v1/code/{}", common::USER2_CODE1_ID))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "display_name": "hacked"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_that!(edit_request.status(), eq(StatusCode::UNAUTHORIZED));
    assert_that!(
        common::convert_response(edit_request).await,
        eq(&json!({
            "message": "Missing authentication. Supply a JWT in the `iceblink_jwt` cookie, or use a bearer in the `Authorization` header.",
            "errorKind": "MissingAuthentication"
        }))
    );

    // Check that it did indeed not happen
    let u2 = common::list_codes_content(&app, a2.as_str()).await;
    assert_that!(u2, common::matchers::code_fixture_default());
}

//
// Code deletion
//

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn delete_code(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, _) = common::get_access_tokens(&db).await;

    let deletion_request = common::delete_code(&app, a1.as_str(), common::USER1_CODE2_ID).await;
    assert_that!(deletion_request.status(), eq(StatusCode::NO_CONTENT));

    let codes_listing = common::list_codes_content(&app, a1.as_str()).await;
    assert_that!(codes_listing.len(), eq(1));

    let remaining_code = codes_listing.get(0).unwrap();
    assert_that!(remaining_code.id, eq(common::USER1_CODE1_ID));
    assert!(remaining_code.is_as_expected());
}

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn delete_code_not_found(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, _) = common::get_access_tokens(&db).await;

    let deletion_request = common::delete_code(&app, a1.as_str(), "random-id").await;
    assert_that!(deletion_request.status(), eq(StatusCode::NOT_FOUND));
    assert_that!(
        common::convert_response(deletion_request).await,
        eq(&json!({
            "message": "Resource not found.",
            "errorKind": "NotFound"
        }))
    );

    // The user codes should not be affected
    let user_codes = common::list_codes_content(&app, a1.as_str()).await;
    assert_that!(user_codes, common::matchers::code_fixture_default());
}

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn delete_code_other_user(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, a2) = common::get_access_tokens(&db).await;

    let deletion_request = common::delete_code(&app, a1.as_str(), common::USER2_CODE1_ID).await;
    assert_that!(deletion_request.status(), eq(StatusCode::NOT_FOUND));
    assert_that!(
        common::convert_response(deletion_request).await,
        eq(&json!({
            "message": "Resource not found.",
            "errorKind": "NotFound"
        }))
    );

    // The victim should not be affected
    let victim_codes = common::list_codes_content(&app, a2.as_str()).await;
    assert_that!(victim_codes, common::matchers::code_fixture_default());
}

//
// Icons
//

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn icon_found_content_type(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, _) = common::get_access_tokens(&db).await;

    let icon_request = common::get_icon(&app, &a1, common::USER1_CODE1_ID).await;
    assert_that!(
        icon_request
            .headers()
            .get(axum::http::header::CONTENT_TYPE)
            .unwrap(),
        eq("image/x-icon")
    );
}

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn icon_code_permission_denied(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, _) = common::get_access_tokens(&db).await;

    let icon_request = common::get_icon(&app, &a1, common::USER2_CODE1_ID).await;
    assert_that!(
        icon_request
            .headers()
            .get(axum::http::header::CONTENT_TYPE)
            .unwrap(),
        eq("application/json")
    );
    assert_that!(icon_request.status(), eq(StatusCode::NOT_FOUND));
    assert_that!(
        common::convert_response(icon_request).await,
        eq(&json!({
            "message": "Resource not found.",
            "errorKind": "NotFound"
        }))
    );
}

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn icon_code_not_found(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, _) = common::get_access_tokens(&db).await;

    let icon_request = common::get_icon(&app, &a1, common::USER2_CODE1_ID).await;
    assert_that!(
        icon_request
            .headers()
            .get(axum::http::header::CONTENT_TYPE)
            .unwrap(),
        eq("application/json")
    );
    assert_that!(icon_request.status(), eq(StatusCode::NOT_FOUND));
    assert_that!(
        common::convert_response(icon_request).await,
        eq(&json!({
            "message": "Resource not found.",
            "errorKind": "NotFound"
        }))
    );
}

#[sqlx::test(fixtures("users", "codes"))]
#[gtest]
async fn icon_returns_same(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, _) = common::get_access_tokens(&db).await;

    let icon_request1 =
        common::convert_response_u8(common::get_icon(&app, &a1, common::USER1_CODE1_ID).await)
            .await;

    let icon_request2 =
        common::convert_response_u8(common::get_icon(&app, &a1, common::USER1_CODE1_ID).await)
            .await;

    assert_that!(icon_request1, eq(&icon_request2));
}

// TODO: Icon Test: it actually using the cached version - not fetching
// TODO: Icon Test: without website url
// TODO: Icon Test: with invalid website url
// TODO: Icon Test: with 404 on favicon
// TODO: Icon Test: what if website returns non-ico?
