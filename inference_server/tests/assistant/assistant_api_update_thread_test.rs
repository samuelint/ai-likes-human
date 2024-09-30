use crate::test_utils;
use app_core::assistant::domain::dto::{
    ApiCreateThreadDto, ApiUpdateThreadDto, MetadataBuilder, ThreadDto,
};
use axum::http::StatusCode;
use serde_json::Value;
use test_utils::router_client::RouterClient;

#[tokio::test]
async fn test_update_thread_metadata() {
    let client = RouterClient::from_app("/openai/v1").await;
    let create_body = ApiCreateThreadDto::default();
    let (response, _) = client
        .post::<ApiCreateThreadDto, ThreadDto>("/threads", &create_body)
        .await
        .unwrap();

    // Updated thread has new metadata
    let mut metadata = MetadataBuilder::create_empty();
    metadata.insert("key".to_string(), Value::String("value".to_string()));

    let update_body = ApiUpdateThreadDto {
        metadata: Some(metadata),
        ..ApiUpdateThreadDto::default()
    };
    let (response, status) = client
        .post::<ApiUpdateThreadDto, ThreadDto>(
            format!("/threads/{}", response.unwrap().id).as_str(),
            &update_body,
        )
        .await
        .unwrap();
    let response = response.unwrap();
    assert_eq!(status, StatusCode::OK, "status should be 200 OK");
    assert_eq!(
        response.metadata.get("key").unwrap().as_str().unwrap(),
        "value"
    );

    // Fetched updated thread has new metadata
    let (response, status) = client
        .get::<ThreadDto>(format!("/threads/{}", response.id).as_str())
        .await
        .unwrap();

    let response = response.unwrap();

    assert_eq!(status, StatusCode::OK, "status should be 200 OK");
    assert_eq!(
        response.metadata.get("key").unwrap().as_str().unwrap(),
        "value"
    );
}
