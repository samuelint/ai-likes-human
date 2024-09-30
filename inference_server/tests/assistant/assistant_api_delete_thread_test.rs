use crate::test_utils::{self, assistant_api::AssistantApiClient};
use app_core::assistant::domain::dto::{
    ApiCreateThreadAndRunDto, ApiCreateThreadDto, ApiCreateThreadMessageDto, ApiMessageContent,
    ThreadDto,
};
use axum::http::StatusCode;
use test_utils::router_client::RouterClient;

#[tokio::test]
async fn test_deleted_thread() {
    let client = RouterClient::from_app("/openai/v1").await;
    let create_body = ApiCreateThreadDto::default();

    let (response, _) = client
        .post::<ApiCreateThreadDto, ThreadDto>("/threads", &create_body)
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
    let create_body = ApiCreateThreadDto::default();

    let (response, _) = client
        .post::<ApiCreateThreadDto, ThreadDto>("/threads", &create_body)
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
    let client = AssistantApiClient::new().await;

    // Create thread with message
    let message1 = ApiCreateThreadMessageDto {
        content: vec![ApiMessageContent::text("Say Hello!")],
        role: "user".to_string(),
        ..ApiCreateThreadMessageDto::default()
    };
    let body = ApiCreateThreadDto {
        messages: vec![message1],
        ..ApiCreateThreadDto::default()
    };

    let (created_thread, _status) = client.create_thread(&body).await;
    let (messages_page, _status) = client.list_thread_messages(&created_thread.id).await;

    let created_messages = messages_page.data;
    let created_message1 = &created_messages[0];

    let _ = client.delete_thread(&created_thread.id).await;

    // Assert message associated with thread is deleted
    let (_, status) = client
        .get_thread_message(&created_thread.id, &created_message1.id)
        .await;

    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "message associated with thread should be deleted with thread"
    );
}

#[tokio::test]
async fn test_deleted_thread_also_deletes_associated_runs() {
    // let client = RouterClient::from_app("/openai/v1").await;
    let client = AssistantApiClient::new().await;

    let create_thread_run_dto = ApiCreateThreadAndRunDto {
        model: "openai:gpt-4o-mini".to_string(),
        ..ApiCreateThreadAndRunDto::default()
    };

    let (run, _) = client.create_thread_and_run(&create_thread_run_dto).await;
    let thread_id = run.thread_id.unwrap();

    let _ = client.delete_thread(&thread_id).await;

    // Assert run associated with thread is deleted
    let (_, status) = client.find_thread_run(&thread_id, &run.id).await;

    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "message associated with thread should be deleted with thread"
    );
}
