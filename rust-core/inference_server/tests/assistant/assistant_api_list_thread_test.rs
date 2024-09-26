use crate::test_utils;
use app_core::assistant::domain::dto::{CreateThreadDto, ThreadDto};
use axum::http::StatusCode;
use test_utils::router_client::RouterClient;

#[tokio::test]
async fn test_when_no_threads_exist_empty_array_is_returned() {
    let client = RouterClient::from_app("/openai/v1").await;

    let (response, status) = client.get::<Vec<ThreadDto>>("/threads").await.unwrap();
    let threads = response.unwrap();

    assert_eq!(status, StatusCode::OK, "status should be 200 OK");
    assert_eq!(threads.len(), 0, "should have 0 threads");
}

#[tokio::test]
async fn test_listing_threads_limit_is_respected() {
    let client = RouterClient::from_app("/openai/v1").await;
    let _ = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &CreateThreadDto::default())
        .await;
    let _ = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &CreateThreadDto::default())
        .await;
    let _ = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &CreateThreadDto::default())
        .await;
    let _ = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &CreateThreadDto::default())
        .await;
    let _ = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &CreateThreadDto::default())
        .await;

    let (response, _status) = client
        .get::<Vec<ThreadDto>>("/threads?limit=4")
        .await
        .unwrap();
    let response = response.unwrap();

    assert_eq!(response.len(), 4, "should have 4 threads");
}
