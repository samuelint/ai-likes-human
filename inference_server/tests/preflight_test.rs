mod test_utils;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use hyper::Method;
// for `collect`
use test_utils::create_test_router;
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn test_preflight_request_are_allowed() {
    let router = create_test_router().await;

    let response = router
        .oneshot(
            Request::builder()
                .method(Method::OPTIONS)
                .uri("/")
                .header("Access-Control-Request-Method", "POST")
                .header("Access-Control-Request-Headers", "Content-Type")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
