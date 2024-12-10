use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use common::AsExpected;
use iceblink_sync::models;
use serde_json::json;
use sqlx::SqlitePool;
use tower::ServiceExt;

pub mod common;

#[sqlx::test(fixtures("users", "codes"))]
async fn list_own_codes(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, a2) = common::get_access_tokens(&db).await;

    let u1 = common::list_codes_content(&app, a1.as_str()).await;
    assert_eq!(u1.len(), 2);
    for code in u1.iter() {
        assert!(code.is_as_expected())
    }

    let u2 = common::list_codes_content(&app, a2.as_str()).await;
    assert_eq!(u2.len(), 1);
    for code in u2.iter() {
        assert!(code.is_as_expected())
    }
}

#[sqlx::test(fixtures("users", "codes"))]
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

    assert_eq!(added.status(), StatusCode::OK);
    let added_res: models::codes::Code =
        serde_json::from_value(common::convert_response(added).await).unwrap();
    assert_eq!(added_res.content, "garbage");
    assert_eq!(added_res.display_name, "Permafrost");
    assert_eq!(added_res.icon_url, None);
    assert_eq!(added_res.website_url, None);
    assert_eq!(added_res.owner_id, common::USER1_ID);
    assert_eq!(added_res.id.len(), 16);

    // Check that it was added to the list
    let listing_request = common::list_codes_content(&app, a1.as_str()).await;
    assert_eq!(listing_request.len(), 3);
    for code in listing_request {
        if code.id == common::USER1_CODE1_ID || code.id == common::USER1_CODE2_ID {
            assert!(code.is_as_expected());
        } else {
            assert_eq!(code.website_url, None);
            assert_eq!(code.icon_url, None);
            assert_eq!(code.content, "garbage");
            assert_eq!(code.owner_id, common::USER1_ID);
            assert_eq!(code.display_name, "Permafrost");
        }
    }

    // User 2 should not be affected by the operation
    let u2 = common::list_codes_content(&app, a2.as_str()).await;
    assert_eq!(u2.len(), 1);
    for code in u2.iter() {
        assert!(code.is_as_expected())
    }
}

#[sqlx::test(fixtures("users", "codes"))]
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

    assert_eq!(added.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        common::convert_response(added).await,
        json!({
            "errorKind": "JsonDataError",
            "message": "Unable to process JSON. Are you missing a field? Tip: Check with the swagger documentation at /swagger!",
        })
    );

    // Check that it was not added to the list
    let listing_request = common::list_codes_content(&app, a1.as_str()).await;
    assert_eq!(listing_request.len(), 2);
    for code in listing_request {
        assert!(code.is_as_expected());
    }
}

//
// Code edit
//

#[sqlx::test(fixtures("users", "codes"))]
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
    assert_eq!(edit_request.status(), StatusCode::OK);
    assert_eq!(
        common::convert_response(edit_request).await,
        json!({
            "content": common::USER1_CODE2_CONTENT,
            "id": common::USER1_CODE2_ID,
            "owner_id": common::USER1_ID,
            "display_name": "google.com",
            "icon_url": null,
            "website_url": null
        })
    );

    // The code is editted in the listing
    let listing_request = common::list_codes_content(&app, a1.as_str()).await;
    assert_eq!(listing_request.len(), 2);
    let unmodified_code = listing_request
        .iter()
        .find(|v| v.id == common::USER1_CODE1_ID)
        .unwrap();
    assert!(unmodified_code.is_as_expected());
    let modified_code = listing_request
        .iter()
        .find(|v| v.id == common::USER1_CODE2_ID)
        .unwrap();
    assert_eq!(modified_code.id, common::USER1_CODE2_ID);
    assert_eq!(modified_code.website_url, None);
    assert_eq!(modified_code.icon_url, None);
    assert_eq!(modified_code.content, common::USER1_CODE2_CONTENT);
    assert_eq!(modified_code.owner_id, common::USER1_ID);
    assert_eq!(modified_code.display_name, "google.com");
}

#[sqlx::test(fixtures("users", "codes"))]
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

    assert_eq!(edit_request.status(), StatusCode::OK);
    assert_eq!(
        common::convert_response(edit_request).await,
        json!({
            "content": common::USER2_CODE1_CONTENT,
            "id": common::USER2_CODE1_ID,
            "owner_id": common::USER2_ID,
            "display_name": "Dummy INC",
            "icon_url": null,
            "website_url": "example.com"
        })
    );

    // The code is editted in the listing
    let listing_request = common::list_codes_content(&app, a2.as_str()).await;
    assert_eq!(listing_request.len(), 1);
    let code = listing_request.get(0).unwrap();
    assert_eq!(code.id, common::USER2_CODE1_ID);
    assert_eq!(code.website_url, Some("example.com".to_string()));
    assert_eq!(code.icon_url, None);
    assert_eq!(code.content, common::USER2_CODE1_CONTENT);
    assert_eq!(code.owner_id, common::USER2_ID);
    assert_eq!(code.display_name, "Dummy INC");
}

#[sqlx::test(fixtures("users", "codes"))]
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
    assert_eq!(edit_request.status(), StatusCode::OK);
    assert_eq!(
        common::convert_response(edit_request).await,
        json!({
            "content": "yippie",
            "id": common::USER1_CODE2_ID,
            "owner_id": common::USER1_ID,
            "display_name": "Modrinth",
            "icon_url": null,
            "website_url": "google.com"
        })
    );

    // The code is editted in the listing
    let listing_request = common::list_codes_content(&app, a1.as_str()).await;
    assert_eq!(listing_request.len(), 2);
    let unmodified_code = listing_request
        .iter()
        .find(|v| v.id == common::USER1_CODE1_ID)
        .unwrap();
    assert!(unmodified_code.is_as_expected());
    let modified_code = listing_request
        .iter()
        .find(|v| v.id == common::USER1_CODE2_ID)
        .unwrap();
    assert_eq!(modified_code.id, common::USER1_CODE2_ID);
    assert_eq!(modified_code.website_url, Some("google.com".to_string()));
    assert_eq!(modified_code.icon_url, None);
    assert_eq!(modified_code.content, "yippie");
    assert_eq!(modified_code.owner_id, common::USER1_ID);
    assert_eq!(modified_code.display_name, "Modrinth");
}

#[sqlx::test(fixtures("users", "codes"))]
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

    assert_eq!(edit_request.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        common::convert_response(edit_request).await,
        json!({
            "message": "Resource not found.",
            "errorKind": "NotFound"
        })
    );
}

#[sqlx::test(fixtures("users", "codes"))]
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

    assert_eq!(edit_request.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        common::convert_response(edit_request).await,
        json!({
            "message": "Resource not found.",
            "errorKind": "NotFound"
        })
    );

    // Check that it did indeed not happen
    let u2 = common::list_codes_content(&app, a2.as_str()).await;
    assert_eq!(u2.len(), 1);
    for code in u2.iter() {
        assert!(code.is_as_expected())
    }
}

#[sqlx::test(fixtures("users", "codes"))]
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

    assert_eq!(edit_request.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        common::convert_response(edit_request).await,
        json!({
            "message": "Missing authentication. Supply a JWT in the `iceblink_jwt` cookie, or use a bearer in the `Authorization` header.",
            "errorKind": "MissingAuthentication"
        })
    );

    // Check that it did indeed not happen
    let u2 = common::list_codes_content(&app, a2.as_str()).await;
    assert_eq!(u2.len(), 1);
    for code in u2.iter() {
        assert!(code.is_as_expected())
    }
}

//
// Code deletion
//

#[sqlx::test(fixtures("users", "codes"))]
async fn delete_code(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, _) = common::get_access_tokens(&db).await;

    let deletion_request = common::delete_code(&app, a1.as_str(), common::USER1_CODE2_ID).await;
    assert_eq!(deletion_request.status(), StatusCode::NO_CONTENT);

    let codes_listing = common::list_codes_content(&app, a1.as_str()).await;
    assert_eq!(codes_listing.len(), 1);
    let code = codes_listing.get(0).unwrap();
    assert_eq!(code.id, common::USER1_CODE1_ID);
    assert!(code.is_as_expected());
}

#[sqlx::test(fixtures("users", "codes"))]
async fn delete_code_not_found(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, _) = common::get_access_tokens(&db).await;

    let deletion_request = common::delete_code(&app, a1.as_str(), "random-id").await;
    assert_eq!(deletion_request.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        common::convert_response(deletion_request).await,
        json!({
            "message": "Resource not found.",
            "errorKind": "NotFound"
        })
    );

    // The user codes should not be affected
    let user_codes = common::list_codes_content(&app, a1.as_str()).await;
    assert_eq!(user_codes.len(), 2);
    for code in user_codes.iter() {
        assert!(code.is_as_expected())
    }
}

#[sqlx::test(fixtures("users", "codes"))]
async fn delete_code_other_user(db: SqlitePool) {
    let app = common::testing_setup(&db).await;
    let (a1, a2) = common::get_access_tokens(&db).await;

    let deletion_request = common::delete_code(&app, a1.as_str(), common::USER2_CODE1_ID).await;
    assert_eq!(deletion_request.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        common::convert_response(deletion_request).await,
        json!({
            "message": "Resource not found.",
            "errorKind": "NotFound"
        })
    );

    // The victim should not be affected
    let victim_codes = common::list_codes_content(&app, a2.as_str()).await;
    assert_eq!(victim_codes.len(), 1);
    for code in victim_codes.iter() {
        assert!(code.is_as_expected())
    }
}

// TODO: Icon Test: getting icon twice returns same
// TODO: Icon Test: it actually using the cached version - not fetching
// TODO: Icon Test: nonexistant code
// TODO: Icon Test: without website url
// TODO: Icon Test: with invalid website url
// TODO: Icon Test: with 404 on favicon
// TODO: Icon Test: with other user's code
// TODO: Icon Test: what if website returns non-ico?
