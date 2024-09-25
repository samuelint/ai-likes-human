use std::sync::Arc;

use crate::chat_completion::{self, ChatCompletionMessageDto, ChatCompletionStream};

use super::dto::ThreadMessageDto;

pub struct ThreadChatCompletionInference {
    inference_service: Arc<chat_completion::InferenceService>,
}

impl ThreadChatCompletionInference {
    pub fn new(inference_service: Arc<chat_completion::InferenceService>) -> Self {
        Self { inference_service }
    }

    pub fn stream(&self, model: &str, messages: &Vec<ThreadMessageDto>) -> ChatCompletionStream {
        let messages: Vec<ChatCompletionMessageDto> =
            messages.iter().map(|m| m.clone().into()).collect();

        self.inference_service.stream(model, &messages)
    }
}
