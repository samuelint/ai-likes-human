use std::sync::Arc;

use crate::chat_completion::{
    self, inference::InferenceArgs, ChatCompletionMessageDto, ChatCompletionStream,
};

use super::dto::ThreadMessageDto;

pub struct ThreadChatCompletionInference {
    inference: Arc<dyn chat_completion::inference::Inference>,
}

impl ThreadChatCompletionInference {
    pub fn new(inference: Arc<dyn chat_completion::inference::Inference>) -> Self {
        Self { inference }
    }

    pub async fn stream(
        &self,
        model: &str,
        messages: &Vec<ThreadMessageDto>,
    ) -> ChatCompletionStream {
        let messages: Vec<ChatCompletionMessageDto> =
            messages.iter().map(|m| m.clone().into()).collect();

        self.inference
            .stream(InferenceArgs {
                model: model.to_string(),
                messages: messages,
                ..Default::default()
            })
            .await
    }
}
