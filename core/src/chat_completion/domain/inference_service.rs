use std::{collections::HashMap, error::Error, sync::Arc};

use super::{
    dto::{ChatCompletionChunkObject, ChatCompletionMessageDto, ChatCompletionObject},
    ChatCompletionResult, ChatCompletionStream,
};
use crate::llm::domain::agent::{base_agent_factory::CreateAgentArgs, AgentFactory};
use async_stream::stream;
use futures::StreamExt;
use langchain_rust::{chain::Chain, prompt_args};

pub struct InferenceService {
    agent_factory: Arc<AgentFactory>,
}

impl InferenceService {
    pub fn new(agent_factory: Arc<AgentFactory>) -> Self {
        Self { agent_factory }
    }

    pub async fn invoke(
        &self,
        model: &str,
        messages: &Vec<ChatCompletionMessageDto>,
    ) -> ChatCompletionResult {
        let input_variables = Self::messages_to_input_variables(messages);
        let chain = self
            .create_chain("default", model)
            .await
            .map_err(|e| e as Box<dyn Error>)?;
        let result = chain.invoke(input_variables).await?;

        let message = ChatCompletionMessageDto::assistant(&result);
        let data = ChatCompletionObject::new_single_choice(message, model);

        Ok(data)
    }

    pub fn stream(
        &self,
        model: &str,
        messages: &Vec<ChatCompletionMessageDto>,
    ) -> ChatCompletionStream {
        let input_variables = Self::messages_to_input_variables(messages);

        let model: String = model.to_string();
        let self_clone = self.clone();

        let stream = stream! {
            let chain = match self_clone.create_chain("default", &model)
                .await {
                    Ok(chain) => chain,
                    Err(e) => {
                        yield Err(e);
                        return;
                    }
                };

            let mut llm_stream = match chain.stream(input_variables).await {
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

    async fn create_chain(
        &self,
        agent_id: &str,
        model: &str,
    ) -> Result<Box<dyn Chain>, Box<dyn Error + Send>> {
        let chain = self
            .agent_factory
            .create(
                agent_id,
                &CreateAgentArgs {
                    model: model.to_string(),
                    ..CreateAgentArgs::default()
                },
            )
            .await?;

        Ok(chain)
    }

    fn messages_to_string(messages: &Vec<ChatCompletionMessageDto>) -> String {
        let messages: Vec<langchain_rust::schemas::Message> =
            messages.iter().map(|m| m.clone().into()).collect();

        langchain_rust::schemas::Message::messages_to_string(&messages)
    }

    fn messages_to_input_variables(
        messages: &Vec<ChatCompletionMessageDto>,
    ) -> HashMap<String, serde_json::Value> {
        let input = Self::messages_to_string(messages);

        prompt_args! {
            "input" => input,
        }
    }
}

impl Clone for InferenceService {
    fn clone(&self) -> Self {
        InferenceService {
            agent_factory: Arc::clone(&self.agent_factory),
        }
    }
}
