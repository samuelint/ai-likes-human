use anyhow::anyhow;
use async_stream::stream;
use futures::StreamExt;
use langchain_rust::{chain::Chain, prompt_args};
use std::{error::Error, sync::Arc};

use crate::{
    chat_completion::domain::langchain_adapter::{
        langchain_messages_to_string, messages_to_langchain_messages,
    },
    llm::domain::agent::{base_agent_factory::CreateAgentArgs, AgentFactory},
};

use super::{
    inference::{Inference, InferenceArgs},
    ChatCompletionChunkObject, ChatCompletionMessageDto, ChatCompletionObject,
    ChatCompletionResult, ChatCompletionStream,
};

pub struct AgentInference {
    agent_factory: Arc<AgentFactory>,
}

impl AgentInference {
    pub fn new(agent_factory: Arc<AgentFactory>) -> Self {
        Self { agent_factory }
    }

    fn get_input_varialbes(
        messages: &[ChatCompletionMessageDto],
    ) -> std::collections::HashMap<String, serde_json::Value> {
        let messages = messages_to_langchain_messages(messages);
        let input = langchain_messages_to_string(&messages);

        prompt_args! {
            "input" => input,
        }
    }

    async fn get_agent(
        &self,
        args: &InferenceArgs,
    ) -> Result<Box<dyn Chain>, Box<dyn Error + Send>> {
        self.agent_factory
            .create(
                "default",
                CreateAgentArgs {
                    model: args.model.to_string(),
                    temperature: args.temperature,
                    ..Default::default()
                },
            )
            .await
    }
}

#[async_trait::async_trait]
impl Inference for AgentInference {
    async fn invoke(&self, args: InferenceArgs) -> ChatCompletionResult {
        let agent = self.get_agent(&args).await?;
        let input_variables = Self::get_input_varialbes(&args.messages);

        let result = agent
            .invoke(input_variables)
            .await
            .map_err(|e| anyhow!(e))?;

        let message = ChatCompletionMessageDto::assistant(&result);
        let data = ChatCompletionObject::new_single_choice(message, &args.model);

        Ok(data)
    }

    async fn stream(&self, args: InferenceArgs) -> ChatCompletionStream {
        let input_variables = Self::get_input_varialbes(&args.messages);
        let model = args.model.to_string();
        let self_clone = self.clone();

        let stream = stream! {
            let agent = match self_clone.get_agent(&args).await {
                Ok(agent) => agent,
                Err(e) => {
                    yield Err(e);
                    return;
                }
            };

            let mut agent_stream = match agent.stream(input_variables).await {
                Ok(stream) => stream,
                Err(e) => {
                    yield Err(Box::new(e));
                    return;
                }
            };

            while let Some(chunk) = agent_stream.next().await {
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

impl Clone for AgentInference {
    fn clone(&self) -> Self {
        Self {
            agent_factory: Arc::clone(&self.agent_factory),
        }
    }
}
