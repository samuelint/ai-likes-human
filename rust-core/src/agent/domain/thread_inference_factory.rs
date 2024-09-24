use std::{convert::Infallible, error::Error, pin::Pin, sync::Arc};

use futures::Stream;

use crate::llm::domain::llm_factory::{CreateLLMParameters, LLMFactory};

use super::dto::{ThreadEvent, ThreadMessageDto};

pub struct ThreadInferenceFactory {
    llm_factory: Arc<dyn LLMFactory>,
}

impl ThreadInferenceFactory {
    pub fn new(llm_factory: Arc<dyn LLMFactory>) -> Self {
        Self { llm_factory }
    }

    pub async fn stream(
        &self,
        model: &str,
        messages: &Vec<ThreadMessageDto>,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<ThreadEvent, Infallible>> + Send + '_>>,
        Box<dyn Error + Send>,
    > {
        let llm = self.llm_factory.create(CreateLLMParameters {
            model: model.to_string(),
        })?;

        let messages: Vec<langchain_rust::schemas::Message> =
            messages.iter().map(|m| m.clone().into()).collect();
        let stream = llm.stream(&messages).await;

        todo!()
    }
}
