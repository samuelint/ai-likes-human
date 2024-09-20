mod test_utils;
use axum::http::StatusCode;
use inference_server::{
    api::types::{ThreadDto, ThreadMessageDto},
    CreateMessageDto, CreateThreadDto,
};
use test_utils::router_client::RouterClient;

#[tokio::test]
async fn test_created_thread_is_successful() {
    let client = RouterClient::from_app("/openai/v1").await;
    let body = CreateThreadDto {
        ..CreateThreadDto::default()
    };

    let (_, status) = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &body)
        .await
        .unwrap();

    assert_eq!(status, StatusCode::OK, "status should be 200 OK");
}

#[tokio::test]
async fn test_created_thread_have_id() {
    let client = RouterClient::from_app("/openai/v1").await;
    let body = CreateThreadDto {
        ..CreateThreadDto::default()
    };

    let (response, _) = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &body)
        .await
        .unwrap();

    assert!(response.id > 0, "should have an id");
}

#[tokio::test]
async fn test_created_thread_have_created_at_date() {
    let client = RouterClient::from_app("/openai/v1").await;
    let body = CreateThreadDto {
        ..CreateThreadDto::default()
    };

    let (response, _) = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &body)
        .await
        .unwrap();

    assert!(response.created_at.len() > 0, "should have a created date");
}

#[tokio::test]
async fn test_created_thread_have_empty_metadata() {
    let client = RouterClient::from_app("/openai/v1").await;

    let body = CreateThreadDto {
        ..CreateThreadDto::default()
    };

    let (response, _) = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &body)
        .await
        .unwrap();
    assert_eq!(response.metadata, "{}", "metadata should be empty");
}

#[tokio::test]
async fn test_created_thread_with_message_is_successful() {
    let client = RouterClient::from_app("/openai/v1").await;
    let message1 = CreateMessageDto {
        content: "Say Hello!".to_string(),
        role: "user".to_string(),
        ..CreateMessageDto::default()
    };
    let body = CreateThreadDto {
        messages: vec![message1],
        ..CreateThreadDto::default()
    };

    let (response, _status) = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &body)
        .await
        .unwrap();
    let (_, status) = client
        .get_as_value(format!("/threads/{}/messages", response.id).as_str())
        .await
        .unwrap();

    assert_eq!(status, StatusCode::OK, "status should be 200 OK");
}

#[tokio::test]
async fn test_created_thread_with_message_have_id() {
    let client = RouterClient::from_app("/openai/v1").await;
    let message1 = CreateMessageDto {
        content: "Say Hello!".to_string(),
        role: "user".to_string(),
        ..CreateMessageDto::default()
    };
    let body = CreateThreadDto {
        messages: vec![message1],
        ..CreateThreadDto::default()
    };

    let (response, _status) = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &body)
        .await
        .unwrap();
    let (response, _) = client
        .get::<Vec<ThreadMessageDto>>(format!("/threads/{}/messages", response.id).as_str())
        .await
        .unwrap();

    assert!(response[0].id > 0, "Message should have an id");
}

#[tokio::test]
async fn test_created_thread_with_message_have_same_content() {
    let client = RouterClient::from_app("/openai/v1").await;
    let message1 = CreateMessageDto {
        content: "Say Hello!".to_string(),
        role: "user".to_string(),
        ..CreateMessageDto::default()
    };
    let body = CreateThreadDto {
        messages: vec![message1],
        ..CreateThreadDto::default()
    };

    let (response, _status) = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &body)
        .await
        .unwrap();
    let (response, _status) = client
        .get::<Vec<ThreadMessageDto>>(format!("/threads/{}/messages", response.id).as_str())
        .await
        .unwrap();

    let first_element = &response[0];
    assert_eq!(first_element.content, "Say Hello!".to_string());
    assert_eq!(first_element.role, "user".to_string());
}
