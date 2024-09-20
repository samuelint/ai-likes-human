mod test_utils;
use axum::http::StatusCode;
use inference_server::{CreateThreadDto, ThreadDto, UpdateThreadDto};
use test_utils::router_client::RouterClient;

#[tokio::test]
async fn test_update_thread_metadata() {
    let client = RouterClient::from_app("/openai/v1").await;
    let create_body = CreateThreadDto::default();
    let (response, _) = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &create_body)
        .await
        .unwrap();

    let update_body = UpdateThreadDto {
        metadata: Some("{\"some\": \"data\"}".to_string()),
        ..UpdateThreadDto::default()
    };
    let (response, status) = client
        .post::<UpdateThreadDto, ThreadDto>(
            format!("/threads/{}", response.id).as_str(),
            &update_body,
        )
        .await
        .unwrap();

    assert_eq!(status, StatusCode::OK, "status should be 200 OK");
    assert_eq!(response.metadata, "{\"some\": \"data\"}".to_string());

    let (response, status) = client
        .get::<ThreadDto>(format!("/threads/{}", response.id).as_str())
        .await
        .unwrap();

    assert_eq!(status, StatusCode::OK, "status should be 200 OK");
    assert_eq!(response.metadata, "{\"some\": \"data\"}".to_string());
}
