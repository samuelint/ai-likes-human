use std::{pin::Pin, sync::Arc};

use futures::Stream;
use futures::StreamExt;
use langchain_rust::{
    language_models::{llm::LLM, GenerateResult, LLMError},
    schemas::{Message, StreamData},
};

use crate::chat_completion::ChatCompletionChoice;
use crate::chat_completion::ChatCompletionMessageDto;
use crate::chat_completion::ChatCompletionObject;

#[derive(Clone)]
pub struct LLamaCppLLM {
    llamacpp: Arc<async_llama_cpp::AsyncLLamaCPP>,
    model: String,
}

impl LLamaCppLLM {
    pub fn new(llamacpp: async_llama_cpp::AsyncLLamaCPP, model: String) -> Self {
        Self {
            llamacpp: Arc::new(llamacpp),
            model,
        }
    }

    fn chunk_to_chat_completion_object(
        chunk: &str,
        finish_reason: Option<String>,
    ) -> ChatCompletionObject {
        let message = ChatCompletionMessageDto::assistant(chunk);
        ChatCompletionObject {
            choices: vec![ChatCompletionChoice {
                index: 0,
                message: Some(message),
                finish_reason,
                ..ChatCompletionChoice::default()
            }],
            ..ChatCompletionObject::default()
        }
    }

    fn create_stream_data(chunk: &str, model: &str, finish_reason: Option<String>) -> StreamData {
        let completion_object = ChatCompletionObject::new_assistant_chunk(&chunk, model, None);
        let value_completion = serde_json::to_value(completion_object)
            .map_err(LLMError::from)
            .unwrap();

        StreamData::new(value_completion, chunk.clone())
    }
}

#[async_trait::async_trait]
impl LLM for LLamaCppLLM {
    async fn generate(&self, messages: &[Message]) -> Result<GenerateResult, LLMError> {
        let chunks: Vec<String> = vec![];

        let mut stream = self.stream(messages).await?;
        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(chunk) => {
                    let chunk = chunk.content;
                    chunks.push(chunk);
                }
                Err(e) => return Err(e),
            }
        }

        let message = chunks.join("");

        Ok(GenerateResult {
            generation: message,
            tokens: None,
        })
    }

    async fn stream(
        &self,
        messages: &[Message],
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamData, LLMError>> + Send>>, LLMError> {
        let prompt = Message::messages_to_string(messages);
        let llamacpp = self.llamacpp.clone();
        let model = self.model.clone();

        let result_stream = async_stream::stream! {
            let mut stream = llamacpp.stream(&self.model, &prompt);
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(chunk) => yield Ok(Self::create_stream_data(&chunk, &model,  None)),
                    Err(e) => yield Err(e),
                };
            }

            yield Ok(Self::create_stream_data("", &model, "stop".to_string().into()));
        };

        Ok(Box::pin(result_stream))
    }
}
