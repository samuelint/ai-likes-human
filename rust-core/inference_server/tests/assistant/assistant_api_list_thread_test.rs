use crate::test_utils;
use app_core::assistant::domain::dto::ThreadDto;
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
