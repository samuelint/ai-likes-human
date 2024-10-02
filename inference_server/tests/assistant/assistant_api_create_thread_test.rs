use crate::test_utils::{self, assistant_api::AssistantApiClient};
use app_core::{
    assistant::domain::dto::{
        ApiCreateThreadDto, ApiCreateThreadMessageDto, MetadataBuilder, PageRequest, ThreadDto,
    },
    chat_completion::ApiMessageContent,
    utils::time::TimeBuilder,
};
use axum::http::StatusCode;
use serde_json::Value;
use test_utils::router_client::RouterClient;

#[tokio::test]
async fn test_created_thread_is_successful() {
    let client = RouterClient::from_app("/openai/v1").await;
    let body = ApiCreateThreadDto {
        ..ApiCreateThreadDto::default()
    };

    let (_, status) = client
        .post::<ApiCreateThreadDto, ThreadDto>("/threads", &body)
        .await
        .unwrap();

    assert_eq!(status, StatusCode::OK, "status should be 200 OK");
}

#[tokio::test]
async fn test_created_thread_have_id() {
    let client = RouterClient::from_app("/openai/v1").await;
    let body = ApiCreateThreadDto {
        ..ApiCreateThreadDto::default()
    };

    let (response, _) = client
        .post::<ApiCreateThreadDto, ThreadDto>("/threads", &body)
        .await
        .unwrap();

    assert!(response.unwrap().id.len() > 0, "should have an id");
}

#[tokio::test]
async fn test_created_thread_have_created_at_date() {
    let client = RouterClient::from_app("/openai/v1").await;
    let body = ApiCreateThreadDto {
        ..ApiCreateThreadDto::default()
    };

    let (response, _) = client
        .post::<ApiCreateThreadDto, ThreadDto>("/threads", &body)
        .await
        .unwrap();

    let created_at = response.unwrap().created_at;
    assert!(created_at > 0, "should have a created date");
    assert!(TimeBuilder::from_i64(created_at).is_near_now());
}

#[tokio::test]
async fn test_created_thread_have_empty_metadata() {
    let client = RouterClient::from_app("/openai/v1").await;

    let body = ApiCreateThreadDto {
        ..ApiCreateThreadDto::default()
    };

    let (response, _) = client
        .post::<ApiCreateThreadDto, ThreadDto>("/threads", &body)
        .await
        .unwrap();
    assert_eq!(
        response.unwrap().metadata.len(),
        0,
        "metadata should be empty"
    );
}

#[tokio::test]
async fn test_created_threads_are_listed() {
    let client = AssistantApiClient::new().await;
    let body = ApiCreateThreadDto {
        ..ApiCreateThreadDto::default()
    };

    let (thread_1, _) = client.create_thread(&body).await;
    let (thread_2, _) = client.create_thread(&body).await;

    let (page, _) = client.list_threads(&PageRequest::default()).await;
    let threads = page.data;

    assert_eq!(threads.len(), 2, "should have 2 threads");
    assert!(
        threads.iter().any(|t| t.id == thread_1.id),
        "should have first thread"
    );
    assert!(
        threads.iter().any(|t| t.id == thread_2.id),
        "should have second thread"
    );
}

#[tokio::test]
async fn test_created_thread_with_message_is_successful() {
    let client = RouterClient::from_app("/openai/v1").await;
    let message1 = ApiCreateThreadMessageDto {
        content: vec![ApiMessageContent::text("Say Hello!")],
        role: "user".to_string(),
        ..ApiCreateThreadMessageDto::default()
    };
    let body = ApiCreateThreadDto {
        messages: vec![message1],
        ..ApiCreateThreadDto::default()
    };

    let (response, _status) = client
        .post::<ApiCreateThreadDto, ThreadDto>("/threads", &body)
        .await
        .unwrap();
    let (_, status) = client
        .get_as_value(format!("/threads/{}/messages", response.unwrap().id).as_str())
        .await
        .unwrap();

    assert_eq!(status, StatusCode::OK, "status should be 200 OK");
}

#[tokio::test]
async fn test_created_thread_with_message_have_id() {
    let client = AssistantApiClient::new().await;

    let message1 = ApiCreateThreadMessageDto {
        content: vec![ApiMessageContent::text("Say Hello!")],
        role: "user".to_string(),
        ..ApiCreateThreadMessageDto::default()
    };
    let body = ApiCreateThreadDto {
        messages: vec![message1],
        ..ApiCreateThreadDto::default()
    };

    let (thread, _) = client.create_thread(&body).await;
    let (page, _) = client.list_thread_messages(&thread.id).await;
    let messages = page.data;

    assert!(messages[0].id.len() > 0, "Message should have an id");
}

#[tokio::test]
async fn test_created_thread_with_message_have_same_content() {
    let client = AssistantApiClient::new().await;
    let message1 = ApiCreateThreadMessageDto {
        content: vec![ApiMessageContent::text("Say Hello!")],
        role: "user".to_string(),
        ..ApiCreateThreadMessageDto::default()
    };
    let body = ApiCreateThreadDto {
        messages: vec![message1],
        ..ApiCreateThreadDto::default()
    };

    let (response, _status) = client.create_thread(&body).await;
    let (page, _status) = client.list_thread_messages(&response.id).await;
    let messages = page.data;

    let first_element = &messages[0];
    assert_eq!(first_element.to_string_content(), "Say Hello!".to_string());
    assert_eq!(first_element.role, "user".to_string());
}

#[tokio::test]
async fn test_created_thread_with_message_can_be_retrieved() {
    let client = AssistantApiClient::new().await;

    let message1 = ApiCreateThreadMessageDto {
        content: vec![ApiMessageContent::text("Say Hello!")],
        role: "user".to_string(),
        ..ApiCreateThreadMessageDto::default()
    };
    let body = ApiCreateThreadDto {
        messages: vec![message1],
        ..ApiCreateThreadDto::default()
    };

    let (created_thread, _) = client.create_thread(&body).await;

    let (page, _status) = client.list_thread_messages(&created_thread.id).await;
    let created_messages = page.data;
    let created_message1 = &created_messages[0];

    let (message, status) = client
        .get_thread_message(&created_thread.id, &created_message1.id)
        .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(message.unwrap().to_string_content(), "Say Hello!");
}

#[tokio::test]
async fn test_created_thread_with_metadata_can_be_retrieved() {
    let client = RouterClient::from_app("/openai/v1").await;
    let mut metadata = MetadataBuilder::create_empty();
    metadata.insert("key".to_string(), Value::String("value".to_string()));

    // Create thread with message
    let body = ApiCreateThreadDto {
        metadata: Some(metadata),
        ..ApiCreateThreadDto::default()
    };
    let (response, status) = client
        .post::<ApiCreateThreadDto, ThreadDto>("/threads", &body)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    let created_thread = response.unwrap();
    assert_eq!(
        created_thread
            .metadata
            .get("key")
            .unwrap()
            .as_str()
            .unwrap(),
        "value"
    );
}
