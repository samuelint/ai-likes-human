mod test_utils;
use app_core::agent::domain::dto::{
    CreateMessageDto, CreateThreadAndRunDto, CreateThreadDto, RunDto,
};
use futures::StreamExt;
use test_utils::{prompt_validator, router_client::RouterClient};

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

// #[tokio::test]
// async fn test_run_is_executed() {
//     let client = RouterClient::from_app("/openai/v1").await;

//     let create_thread_run_dto = CreateThreadAndRunDto {
//         model: "openai:gpt-4o-mini".to_string(),
//         thread: CreateThreadDto {
//             messages: vec![CreateMessageDto {
//                 content: "Tell me a joke".to_string(),
//                 ..CreateMessageDto::user()
//             }],
//             ..CreateThreadDto::default()
//         },
//         stream: Some(true),
//         ..CreateThreadAndRunDto::default()
//     };

//     let mut stream =
//         client.post_stream::<CreateThreadAndRunDto>("/threads/runs", &create_thread_run_dto);

//     let mut responses = Vec::new();
//     while let Some(chunk) = stream.next().await {
//         let response = chunk.unwrap();
//         responses.push(response);
//     }
//     let response: String = responses.join(" - ");

//     let is_response_a_joke =
//         prompt_validator::yes_or_no(format!("Is this a joke? \"{}\"", response.as_str())).await;
//     assert!(is_response_a_joke, "Response should be a joke")
// }
