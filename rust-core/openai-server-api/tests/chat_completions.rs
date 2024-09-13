mod test_utils;

use futures::stream::StreamExt;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_server_api::client::ChatCompletionsStreamClient;
use std::env;
use test_utils::with_started_server;

static API_ENDPOINT: &str = "http://localhost:1234/openai/v1";

#[tokio::test]
async fn test_chat_completions() {
    with_started_server(async move {
        let client = OpenAIClient::new_with_endpoint(
            API_ENDPOINT.to_string(),
            env::var("OPENAI_API_KEY").unwrap().to_string(),
        );

        let req = ChatCompletionRequest::new(
            "gpt-4o-mini".to_string(),
            vec![chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text(String::from("What is bitcoin?")),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            }],
        );

        let result = client.chat_completion(req).await.unwrap();
        let r_content = result.choices[0].message.content.as_slice();
        assert!(!r_content.is_empty());
    })
    .await;
}

#[tokio::test]
async fn test_stream_chat_completions() {
    with_started_server(async move {
        let openai_stream = ChatCompletionsStreamClient::new_with_api_url(
            env::var("OPENAI_API_KEY").unwrap().to_string(),
            format!("{}/chat/completions", API_ENDPOINT),
        );

        let input = r#"
        {
            "model": "gpt-4o-mini",
            "messages": [
                {
                    "role": "user",
                    "content": "Write a simple advanced usage of Rust in one sentence"
                }
            ]
        }
    "#;

        let stream = openai_stream.stream(input).await.unwrap();
        let mut stream = Box::pin(stream);

        let mut responses = Vec::new();
        while let Some(response) = stream.next().await {
            responses.push(response);
        }
        let response: String = responses.join(" - ");
        println!("Response: {}", response);
        assert!(!response.is_empty());
    })
    .await;
}
