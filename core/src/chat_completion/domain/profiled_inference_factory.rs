use std::{error::Error, sync::Arc};

use async_stream::stream;

use super::{
    inference::{Inference, InferenceArgs},
    ChatCompletionMessageDto, ChatCompletionResult, ChatCompletionStream,
};
use crate::profile::domain::ProfileSystemPromptFactory;
use futures::StreamExt;

pub struct ProfiledInferenceFactory {
    inference: Arc<dyn Inference>,
    system_prompt_factory: Arc<ProfileSystemPromptFactory>,
}

impl ProfiledInferenceFactory {
    pub fn new(
        inference: Arc<dyn Inference>,
        system_prompt_factory: Arc<ProfileSystemPromptFactory>,
    ) -> Self {
        Self {
            inference,
            system_prompt_factory,
        }
    }

    async fn args_with_system_prompt(
        &self,
        args: InferenceArgs,
    ) -> Result<InferenceArgs, Box<dyn Error + Send>> {
        let system_prompt = self.system_prompt_factory.create_system_prompt().await?;
        let system_message = ChatCompletionMessageDto::system(&system_prompt);

        let mut messages = args.messages.clone();
        messages.insert(0, system_message);

        Ok(InferenceArgs {
            messages,
            ..args.clone()
        })
    }
}

#[async_trait::async_trait]
impl Inference for ProfiledInferenceFactory {
    async fn invoke(&self, args: InferenceArgs) -> ChatCompletionResult {
        let args_with_system_prompt = self.args_with_system_prompt(args.clone()).await?;
        self.inference.invoke(args_with_system_prompt).await
    }

    async fn stream(&self, args: InferenceArgs) -> ChatCompletionStream {
        let inference = self.inference.clone();
        let self_clone = self.clone();

        let stream = stream! {
            let args_with_system_prompt = match self_clone.args_with_system_prompt(args.clone()).await {
                Ok(args_with_system_prompt) => args_with_system_prompt,
                Err(e) => {
                    yield Err(e);
                    return;
                }
            };

            let mut inference_stream = inference.stream(args_with_system_prompt).await;

            while let Some(chunk) = inference_stream.next().await {
                yield chunk;
            }
        };

        Box::pin(stream)
    }
}

impl Clone for ProfiledInferenceFactory {
    fn clone(&self) -> Self {
        Self {
            inference: Arc::clone(&self.inference),
            system_prompt_factory: Arc::clone(&self.system_prompt_factory),
        }
    }
}
