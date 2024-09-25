use crate::test_utils;
use app_core::assistant::domain::dto::{
    CreateMessageDto, CreateThreadDto, ThreadDto, ThreadMessageDto,
};
use axum::http::StatusCode;
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

    assert!(response.unwrap().id.len() > 0, "should have an id");
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

    assert!(
        response.unwrap().created_at.len() > 0,
        "should have a created date"
    );
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
    assert_eq!(response.unwrap().metadata, "{}", "metadata should be empty");
}

#[tokio::test]
async fn test_created_threads_are_listed() {
    let client = RouterClient::from_app("/openai/v1").await;
    let body = CreateThreadDto {
        ..CreateThreadDto::default()
    };

    let (response, _) = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &body)
        .await
        .unwrap();
    let thread_1 = response.unwrap();

    let (response, _) = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &body)
        .await
        .unwrap();
    let thread_2 = response.unwrap();

    let (response, _) = client.get::<Vec<ThreadDto>>("/threads").await.unwrap();
    let threads = response.unwrap();

    assert_eq!(threads.len(), 2, "should have 2 threads");
    assert_eq!(threads[0].id, thread_1.id, "should have first thread");
    assert_eq!(threads[1].id, thread_2.id, "should have second thread");
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
        .get_as_value(format!("/threads/{}/messages", response.unwrap().id).as_str())
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

    let thread = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &body)
        .await
        .unwrap()
        .0
        .unwrap();
    let messages = client
        .get::<Vec<ThreadMessageDto>>(format!("/threads/{}/messages", thread.id).as_str())
        .await
        .unwrap()
        .0
        .unwrap();

    assert!(messages[0].id.len() > 0, "Message should have an id");
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
        .get::<Vec<ThreadMessageDto>>(
            format!("/threads/{}/messages", response.unwrap().id).as_str(),
        )
        .await
        .unwrap();

    let first_element = &response.unwrap()[0];
    assert_eq!(first_element.content, "Say Hello!".to_string());
    assert_eq!(first_element.role, "user".to_string());
}

#[tokio::test]
async fn test_created_thread_with_message_can_be_retrieved() {
    let client = RouterClient::from_app("/openai/v1").await;

    // Create thread with message
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
    let created_thread = response.unwrap();
    let (response, _status) = client
        .get::<Vec<ThreadMessageDto>>(format!("/threads/{}/messages", created_thread.id).as_str())
        .await
        .unwrap();
    let created_messages = response.unwrap();
    let created_message1 = &created_messages[0];

    // Assert message associated with thread is found
    let (response, status) = client
        .get::<ThreadMessageDto>(
            format!(
                "/threads/{}/messages/{}",
                created_thread.id, created_message1.id
            )
            .as_str(),
        )
        .await
        .unwrap();
    let message = response.unwrap();

    assert_eq!(status, StatusCode::OK);
    assert_eq!(message.content, "Say Hello!");
}
