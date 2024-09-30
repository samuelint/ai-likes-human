use std::{error::Error, sync::Arc};

use async_stream::stream;

use crate::llm::domain::llm_factory::{CreateLLMParameters, LLMFactory};

use super::{
    dto::{ChatCompletionChunkObject, ChatCompletionMessageDto, ChatCompletionObject},
    ChatCompletionResult, ChatCompletionStream,
};
use futures::StreamExt;

pub struct InferenceService {
    llm_factory: Arc<dyn LLMFactory>,
}

impl InferenceService {
    pub fn new(llm_factory: Arc<dyn LLMFactory>) -> Self {
        Self { llm_factory }
    }

    pub async fn invoke(
        &self,
        model: &str,
        messages: &Vec<ChatCompletionMessageDto>,
    ) -> ChatCompletionResult {
        let messages: Vec<langchain_rust::schemas::Message> =
            messages.iter().map(|m| m.clone().into()).collect();
        let llm = self
            .llm_factory
            .create(&CreateLLMParameters {
                model: model.to_string(),
                ..CreateLLMParameters::default()
            }).await
            .map_err(|e| e as Box<dyn Error>)?;

        let result = llm.generate(&messages[..]).await?;

        let message = ChatCompletionMessageDto::new_assistant(&result.generation);
        let data = ChatCompletionObject::new_single_choice(message, model);

        Ok(data)
    }

    pub fn stream(
        &self,
        model: &str,
        messages: &Vec<ChatCompletionMessageDto>,
    ) -> ChatCompletionStream {
        let messages: Vec<langchain_rust::schemas::Message> =
            messages.iter().map(|m| m.clone().into()).collect();

        let model = model.to_string();
        let llm_factory = Arc::clone(&self.llm_factory);
        let stream = stream! {
            let llm = match llm_factory.create(&CreateLLMParameters {
                model: model.clone(),
                ..CreateLLMParameters::default()
            }).await {
                Ok(llm) => llm,
                Err(e) => {
                    yield Err(e);
                    return;
                }
            };

            let mut llm_stream = match llm.stream(&messages[..]).await {
                Ok(stream) => stream,
                Err(e) => {
                    yield Err(Box::new(e));
                    return;
                }
            };

            while let Some(chunk) = llm_stream.next().await {
                let chunk = chunk.unwrap();
                let chunk = ChatCompletionChunkObject::new_assistant_chunk(&chunk.content, &model);

                yield Ok(chunk);
            }
        };

        Box::pin(stream)
    }
}
