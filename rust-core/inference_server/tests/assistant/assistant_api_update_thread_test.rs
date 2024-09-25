use crate::test_utils;
use app_core::assistant::domain::dto::{CreateThreadDto, ThreadDto, UpdateThreadDto};
use axum::http::StatusCode;
use test_utils::router_client::RouterClient;

#[tokio::test]
async fn test_update_thread_metadata() {
    let client = RouterClient::from_app("/openai/v1").await;
    let create_body = CreateThreadDto::default();
    let (response, _) = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &create_body)
        .await
        .unwrap();

    // Updated thread has new metadata
    let update_body = UpdateThreadDto {
        metadata: Some("{\"some\": \"data\"}".to_string()),
        ..UpdateThreadDto::default()
    };
    let (response, status) = client
        .post::<UpdateThreadDto, ThreadDto>(
            format!("/threads/{}", response.unwrap().id).as_str(),
            &update_body,
        )
        .await
        .unwrap();
    let response = response.unwrap();
    assert_eq!(status, StatusCode::OK, "status should be 200 OK");
    assert_eq!(response.metadata, "{\"some\": \"data\"}".to_string());

    // Fetched updated thread has new metadata
    let (response, status) = client
        .get::<ThreadDto>(format!("/threads/{}", response.id).as_str())
        .await
        .unwrap();

    assert_eq!(status, StatusCode::OK, "status should be 200 OK");
    assert_eq!(
        response.unwrap().metadata,
        "{\"some\": \"data\"}".to_string()
    );
}
