use crate::test_utils;
use app_core::assistant::domain::dto::{CreateRunDto, CreateThreadDto, RunDto, ThreadDto};
use axum::http::StatusCode;
use test_utils::router_client::RouterClient;

#[tokio::test]
async fn test_created_thread_run_is_successful() {
    let client = RouterClient::from_app("/openai/v1").await;
    let thread = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &CreateThreadDto::default())
        .await
        .unwrap()
        .0
        .unwrap();

    let create_thread_run_dto = CreateRunDto {
        ..CreateRunDto::default()
    };

    let (_, status) = client
        .post::<CreateRunDto, RunDto>(
            format!("/threads/{}/runs", thread.id).as_str(),
            &create_thread_run_dto,
        )
        .await
        .unwrap();

    assert_eq!(status, StatusCode::OK, "status should be 200 OK");
}

#[tokio::test]
async fn test_created_thread_run_has_created_at() {
    let client = RouterClient::from_app("/openai/v1").await;
    let thread = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &CreateThreadDto::default())
        .await
        .unwrap()
        .0
        .unwrap();

    let create_thread_run_dto = CreateRunDto {
        ..CreateRunDto::default()
    };

    let run = client
        .post::<CreateRunDto, RunDto>(
            format!("/threads/{}/runs", thread.id).as_str(),
            &create_thread_run_dto,
        )
        .await
        .unwrap()
        .0
        .unwrap();
    assert!(run.created_at.len() > 0);

    let retrieved_run = client
        .get::<RunDto>(format!("/threads/{}/runs/{}", thread.id, run.id).as_str())
        .await
        .unwrap()
        .0
        .unwrap();

    assert_eq!(retrieved_run.created_at, run.created_at);
}

#[tokio::test]
async fn test_created_thread_run_has_model() {
    let client = RouterClient::from_app("/openai/v1").await;
    let thread = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &CreateThreadDto::default())
        .await
        .unwrap()
        .0
        .unwrap();

    let create_thread_run_dto = CreateRunDto {
        model: "openai:gpt-4o-mini".to_string(),
        ..CreateRunDto::default()
    };

    let run = client
        .post::<CreateRunDto, RunDto>(
            format!("/threads/{}/runs", thread.id).as_str(),
            &create_thread_run_dto,
        )
        .await
        .unwrap()
        .0
        .unwrap();
    assert_eq!(run.model, "openai:gpt-4o-mini");

    let retrieved_run = client
        .get::<RunDto>(format!("/threads/{}/runs/{}", thread.id, run.id).as_str())
        .await
        .unwrap()
        .0
        .unwrap();

    assert_eq!(retrieved_run.model, run.model);
}

#[tokio::test]
async fn test_created_thread_run_status_is_queued() {
    let client = RouterClient::from_app("/openai/v1").await;
    let thread = client
        .post::<CreateThreadDto, ThreadDto>("/threads", &CreateThreadDto::default())
        .await
        .unwrap()
        .0
        .unwrap();

    let create_thread_run_dto = CreateRunDto {
        model: "openai:gpt-4o-mini".to_string(),
        ..CreateRunDto::default()
    };

    let run = client
        .post::<CreateRunDto, RunDto>(
            format!("/threads/{}/runs", thread.id).as_str(),
            &create_thread_run_dto,
        )
        .await
        .unwrap()
        .0
        .unwrap();
    assert_eq!(run.status, "queued");
}
