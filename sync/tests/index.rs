// use iceblink_sync::configure_router;

// #[tokio::test]
// async fn index() {
//     let app = configure_router();

//     let response = app
//         .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
//         .await
//         .unwrap();

//     assert_eq!(response.status(), StatusCode::OK);

//     let body = response.into_body().collect().await.unwrap().to_bytes();
//     assert_eq!(&body[..], b"Hello, World!");
// }
