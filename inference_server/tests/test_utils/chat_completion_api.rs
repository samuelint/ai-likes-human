use std::pin::Pin;

use app_core::chat_completion::{ChatCompletionChunkObject, ChatCompletionObject};
use futures::{Stream, StreamExt};
use inference_server::ApiChatCompletionRequestDto;

use super::router_client::RouterClient;

#[allow(dead_code)]
pub static DEFAULT_LLM_MODEL: &str = "openai:gpt-4o-mini";

#[allow(dead_code)]
pub struct ChatCompletionApiClient {
    client: RouterClient,
}

impl ChatCompletionApiClient {
    #[allow(dead_code)]
    pub async fn new() -> Self {
        let client = RouterClient::from_app("/openai/v1").await;

        Self { client }
    }

    #[allow(dead_code)]
    pub async fn invoke(&self, request: &ApiChatCompletionRequestDto) -> ChatCompletionObject {
        let (dto, _status) = self
            .client
            .post::<ApiChatCompletionRequestDto, ChatCompletionObject>("/chat/completions", request)
            .await
            .unwrap();

        dto.unwrap()
    }

    #[allow(dead_code)]
    pub async fn stream(
        &self,
        request: &ApiChatCompletionRequestDto,
    ) -> impl Stream<Item = Result<(String, ChatCompletionChunkObject), axum::Error>> {
        let stream = self
            .client
            .post_json_stream::<ApiChatCompletionRequestDto, ChatCompletionChunkObject>(
                "/chat/completions",
                &ApiChatCompletionRequestDto {
                    stream: Some(true),
                    ..request.clone()
                },
            );

        stream
    }

    #[allow(dead_code)]
    pub async fn stream_as_events_vec(
        &self,
        request: &ApiChatCompletionRequestDto,
    ) -> Vec<ChatCompletionChunkObject> {
        let stream = self.stream(request).await;

        Self::stream_to_events_vec(Box::pin(stream)).await
    }

    pub async fn stream_to_events_vec(
        mut stream: Pin<
            Box<dyn Stream<Item = Result<(String, ChatCompletionChunkObject), axum::Error>>>,
        >,
    ) -> Vec<ChatCompletionChunkObject> {
        let mut responses = Vec::new();
        while let Some(chunk) = stream.next().await {
            let response = chunk.unwrap();
            responses.push(response.1);
        }

        responses
    }
}
