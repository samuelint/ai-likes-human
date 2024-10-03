use std::sync::Arc;

use crate::{
    chat_completion::{ChatCompletionMessageDto, ChatCompletionObject},
    llm::domain::llm_factory::{CreateLLMParameters, LLMFactory},
};
use anyhow::anyhow;
use async_stream::stream;
use futures::StreamExt;

use super::{
    inference::{Inference, InferenceArgs},
    langchain_adapter::messages_to_langchain_messages,
    ChatCompletionChunkObject, ChatCompletionResult, ChatCompletionStream,
};

pub struct LLMInference {
    llm_factory: Arc<dyn LLMFactory>,
}

impl LLMInference {
    pub fn new(llm_factory: Arc<dyn LLMFactory>) -> Self {
        Self { llm_factory }
    }
}

#[async_trait::async_trait]
impl Inference for LLMInference {
    async fn invoke(&self, args: InferenceArgs) -> ChatCompletionResult {
        let model = &args.model;
        let messages = messages_to_langchain_messages(&args.messages);

        let llm = self
            .llm_factory
            .create(&CreateLLMParameters {
                model: model.to_string(),
                temperature: args.temperature,
            })
            .await?;

        let result = llm.generate(&messages[..]).await.map_err(|e| anyhow!(e))?;

        let message = ChatCompletionMessageDto::assistant(&result.generation);
        let data = ChatCompletionObject::new_single_choice(message, model);

        Ok(data)
    }

    async fn stream(&self, args: InferenceArgs) -> ChatCompletionStream {
        let messages: Vec<langchain_rust::schemas::Message> =
            args.messages.iter().map(|m| m.clone().into()).collect();

        let model = args.model.to_string();
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
                let chunk = match chunk {
                    Ok(chunk) => chunk,
                    Err(e) => {
                        yield Err(Box::new(e));
                        return;
                    }
                };
                let chunk = ChatCompletionChunkObject::new_assistant_chunk(&chunk.content, &model);

                yield Ok(chunk);
            }
        };

        Box::pin(stream)
    }
}
