mod test_utils;
use app_core::agent::domain::dto::{
    CreateMessageDto, CreateThreadAndRunDto, CreateThreadDto, RunDto, ThreadDto, ThreadMessageDto,
};
use axum::http::StatusCode;
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

#[tokio::test]
async fn test_deleted_thread_also_deletes_associated_messages() {
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

    // Delete thread
    client
        .delete(format!("/threads/{}", created_thread.id).as_str())
        .await
        .unwrap();

    // Assert message associated with thread is deleted
    let (_, status) = client
        .get::<ThreadMessageDto>(
            format!(
                "/threads/{}/messages/{}",
                created_thread.id, created_message1.id
            )
            .as_str(),
        )
        .await
        .unwrap();

    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "message associated with thread should be deleted with thread"
    );
}

#[tokio::test]
async fn test_deleted_thread_also_deletes_associated_runs() {
    let client = RouterClient::from_app("/openai/v1").await;
    let create_thread_run_dto = CreateThreadAndRunDto {
        model: "openai:gpt-4o-mini".to_string(),
        ..CreateThreadAndRunDto::default()
    };

    let run = client
        .post::<CreateThreadAndRunDto, RunDto>("/threads/runs", &create_thread_run_dto)
        .await
        .unwrap()
        .0
        .unwrap();

    let thread_id = run.thread_id.unwrap();

    // Delete thread
    client
        .delete(format!("/threads/{}", thread_id).as_str())
        .await
        .unwrap();

    // Assert run associated with thread is deleted
    let (_, status) = client
        .get::<RunDto>(format!("/threads/{}/runs/{}", thread_id, run.id).as_str())
        .await
        .unwrap();

    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "message associated with thread should be deleted with thread"
    );
}
