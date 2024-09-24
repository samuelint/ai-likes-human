mod test_utils;
use app_core::agent::domain::dto::{
    CreateMessageDto, CreateThreadAndRunDto, CreateThreadDto, ThreadEvent,
};
use futures::{Stream, StreamExt};
use test_utils::router_client::RouterClient;

static LLM_MODEL: &str = "openai:gpt-4o-mini";

async fn create_run_stream(prompt: &str) -> impl Stream<Item = Result<ThreadEvent, axum::Error>> {
    let client = RouterClient::from_app("/openai/v1").await;

    let create_thread_run_dto = CreateThreadAndRunDto {
        model: LLM_MODEL.to_string(),
        thread: CreateThreadDto {
            messages: vec![CreateMessageDto {
                content: prompt.to_string(),
                ..CreateMessageDto::user()
            }],
            ..CreateThreadDto::default()
        },
        stream: Some(true),
        ..CreateThreadAndRunDto::default()
    };

    let stream = client.post_json_stream::<CreateThreadAndRunDto, ThreadEvent>(
        "/threads/runs",
        &create_thread_run_dto,
    );

    stream
}

async fn stream_as_chunks_array(prompt: &str) -> Vec<ThreadEvent> {
    let mut stream = create_run_stream(prompt).await;

    let mut responses = Vec::new();
    while let Some(chunk) = stream.next().await {
        let response = chunk.unwrap();
        responses.push(response);
    }

    responses
}

#[tokio::test]
async fn test_run_stream_starts_with_thread_created() {
    let chunks = stream_as_chunks_array("Tell me a joke.").await;
    let chunk = &chunks[0];

    let chunk = match chunk {
        ThreadEvent::ThreadCreated(event) => event,
        _ => panic!("Expected ThreadCreated event"),
    };

    assert_eq!(chunk.event, "thread.created");
}

#[tokio::test]
async fn test_run_stream_thread_created_contains_new_thread_data() {
    let chunks = stream_as_chunks_array("Tell me a joke.").await;
    let chunk = &chunks[0];

    let chunk = match chunk {
        ThreadEvent::ThreadCreated(event) => event,
        _ => panic!("Expected ThreadCreated event"),
    };

    assert!(chunk.data.id.len() > 0, "Should have a thread id");
    assert!(
        chunk.data.created_at.len() > 0,
        "Should have a created at date"
    );
}

#[tokio::test]
async fn test_run_stream_second_chunk_is_thread_run_created() {
    let chunks = stream_as_chunks_array("Tell me a joke.").await;
    let chunk = &chunks[1];

    let chunk = match chunk {
        ThreadEvent::ThreadRunCreated(event) => event,
        _ => panic!("Expected ThreadRunCreated event"),
    };

    assert_eq!(chunk.event, "thread.run.created");
}

#[tokio::test]
async fn test_run_stream_thread_run_created_chunk_contains_run_data() {
    let chunks = stream_as_chunks_array("Tell me a joke.").await;
    let chunk = &chunks[1];

    let chunk = match chunk {
        ThreadEvent::ThreadRunCreated(event) => event,
        _ => panic!("Expected ThreadRunCreated event"),
    };

    assert!(chunk.data.id.len() > 0, "Should have a thread id");
    assert!(
        chunk.data.created_at.len() > 0,
        "Should have a created at date"
    );
}

#[tokio::test]
async fn test_run_stream_thread_run_created_chunk_contains_run_status_is_queued() {
    let chunks = stream_as_chunks_array("Tell me a joke.").await;
    let chunk = &chunks[1];

    let chunk = match chunk {
        ThreadEvent::ThreadRunCreated(event) => event,
        _ => panic!("Expected ThreadRunCreated event"),
    };

    assert_eq!(chunk.data.status, "queued");
}

#[tokio::test]
async fn test_run_stream_thread_run_created_chunk_contains_llm_model() {
    let chunks = stream_as_chunks_array("Tell me a joke.").await;
    let chunk = &chunks[1];

    let chunk = match chunk {
        ThreadEvent::ThreadRunCreated(event) => event,
        _ => panic!("Expected ThreadRunCreated event"),
    };

    assert_eq!(chunk.data.model, LLM_MODEL);
}
