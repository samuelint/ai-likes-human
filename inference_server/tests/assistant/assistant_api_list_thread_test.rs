use crate::test_utils::assistant_api::AssistantApiClient;
use app_core::assistant::domain::dto::{ApiCreateThreadDto, PageRequest};
use axum::http::StatusCode;

#[tokio::test]
async fn test_when_no_threads_exist_empty_array_is_returned() {
    let client = AssistantApiClient::new().await;
    let (response, status) = client.list_threads(&PageRequest::default()).await;
    let threads = response.data;

    assert_eq!(status, StatusCode::OK, "status should be 200 OK");
    assert_eq!(threads.len(), 0, "should have 0 threads");
}

#[tokio::test]
async fn test_listing_threads_limit_is_respected() {
    let client = AssistantApiClient::new().await;

    let _ = client.create_thread(&ApiCreateThreadDto::default()).await;
    let _ = client.create_thread(&ApiCreateThreadDto::default()).await;
    let _ = client.create_thread(&ApiCreateThreadDto::default()).await;
    let _ = client.create_thread(&ApiCreateThreadDto::default()).await;

    let (response, _status) = client
        .list_threads(&PageRequest {
            limit: Some(2.to_string()),
            ..PageRequest::default()
        })
        .await;

    let threads = response.data;
    assert_eq!(threads.len(), 2, "should have 2 threads");
}
