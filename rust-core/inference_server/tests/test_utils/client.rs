use std::pin::Pin;

use openai_api_rs::v1::{
    api::OpenAIClient,
    chat_completion::{self, ChatCompletionRequest, ChatCompletionResponse},
};

use super::stream_chat_completions::{ChatCompletionsStreamClient, OpenAIStream};

pub struct OpenaiClient {
    invoke_client: OpenAIClient,
    stream_client: ChatCompletionsStreamClient,
}

impl OpenaiClient {
    #[allow(dead_code)]
    pub fn new(server_url: String) -> Self {
        let invoke_client = OpenAIClient::new_with_endpoint(server_url.clone(), "any".to_string());
        let stream_client = ChatCompletionsStreamClient::new_with_api_url(
            "any".to_string(),
            format!("{}/chat/completions", server_url.clone()),
        );

        OpenaiClient {
            invoke_client,
            stream_client,
        }
    }

    #[allow(dead_code)]
    pub async fn user_invoke(&self, model: &str, message: &str) -> ChatCompletionResponse {
        let req = ChatCompletionRequest::new(
            String::from(model),
            vec![chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text(String::from(message)),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            }],
        );

        self.invoke_client.chat_completion(req).await.unwrap()
    }

    #[allow(dead_code)]
    pub async fn user_stream(&self, model: &str, message: &str) -> Pin<Box<OpenAIStream>> {
        let input = format!(
            r#"
            {{
                "model": "{}",
                "messages": [
                    {{
                        "role": "user",
                        "content": "{}"
                    }}
                ]
            }}
        "#,
            model, message
        );

        let stream = self.stream_client.stream(input.as_str()).await.unwrap();

        Box::pin(stream)
    }
}
