use mockall::automock;

use crate::chat_completion::{
    ChatCompletionMessageDto, ChatCompletionResult, ChatCompletionStream,
};

#[derive(Default)]
pub struct InferenceArgs {
    pub model: String,
    pub temperature: Option<f32>,
    pub messages: Vec<ChatCompletionMessageDto>,
}

#[automock]
#[async_trait::async_trait]
pub trait Inference: Send + Sync {
    async fn invoke(&self, args: InferenceArgs) -> ChatCompletionResult;
    async fn stream(&self, args: InferenceArgs) -> ChatCompletionStream;
}
