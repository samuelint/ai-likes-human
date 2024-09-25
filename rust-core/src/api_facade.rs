use std::sync::Arc;

use crate::{
    chat_completion::{ChatCompletionMessageDto, ChatCompletionResult, ChatCompletionStream},
    AppContainer,
};

pub struct ApiFacade {
    container: Arc<AppContainer>,
}

impl ApiFacade {
    pub fn new(container: Arc<AppContainer>) -> Self {
        Self { container }
    }

    pub async fn chat_completion_invoke(
        &self,
        model: &str,
        messages: &Vec<ChatCompletionMessageDto>,
    ) -> ChatCompletionResult {
        let factory = self
            .container
            .chat_completion_module
            .get_inference_factory();

        factory.invoke(model, messages).await
    }

    pub fn chat_completion_stream(
        &self,
        model: &str,
        messages: &Vec<ChatCompletionMessageDto>,
    ) -> ChatCompletionStream {
        let factory = self
            .container
            .chat_completion_module
            .get_inference_factory();

        factory.stream(model, messages)
    }
}
