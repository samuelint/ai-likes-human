mod test_utils;
use axum::http::StatusCode;
use inference_server::{CreateThreadDto, ThreadDto};
use test_utils::router_client::RouterClient;

#[tokio::test]
async fn test_deleted_thread() {
    let client = RouterClient::from_app("/openai/v1").await;
    let create_body = CreateThreadDto::default();

    let (response, _) = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &create_body)
        .await
        .unwrap();
    let response = response.unwrap();

    let status = client
        .delete(format!("/threads/{}", response.id).as_str())
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK, "status should be 200 OK");
}

#[tokio::test]
async fn test_deleted_thread_cannot_be_fetched_again() {
    let client = RouterClient::from_app("/openai/v1").await;
    let create_body = CreateThreadDto::default();

    let (response, _) = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &create_body)
        .await
        .unwrap();
    let response = response.unwrap();
    client
        .delete(format!("/threads/{}", response.id).as_str())
        .await
        .unwrap();

    let (response, status) = client
        .get::<ThreadDto>(format!("/threads/{}", response.id).as_str())
        .await
        .unwrap();

    assert_eq!(status, StatusCode::NOT_FOUND, "thread should not be found");
    assert!(response.is_none());
}
