mod test_utils;
use app_core::agent::domain::dto::{CreateThreadAndRunDto, RunDto};
use test_utils::router_client::RouterClient;

#[tokio::test]
async fn test_create_thread_and_run_has_thread_id() {
    let client = RouterClient::from_app("/openai/v1").await;

    let create_thread_run_dto = CreateThreadAndRunDto {
        model: "openai:gpt-4o-mini".to_string(),
        stream: Some(false),
        ..CreateThreadAndRunDto::default()
    };

    let run = client
        .post::<CreateThreadAndRunDto, RunDto>("/threads/runs", &create_thread_run_dto)
        .await
        .unwrap()
        .0
        .unwrap();

    assert!(run.thread_id.unwrap().len() > 0, "thread id should be > 0");
    assert!(run.id.len() > 0, "run id should be > 0");
}
