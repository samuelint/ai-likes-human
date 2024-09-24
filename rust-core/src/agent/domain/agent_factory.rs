use std::{error::Error, sync::Arc};

use crate::llm::domain::llm_factory::{CreateLLMParameters, LLMFactory};

use langchain_rust::{
    agent::{AgentExecutor, ConversationalAgentBuilder},
    chain::Chain,
    language_models::llm::LLM,
};

pub struct CreateAgentParameters {
    pub model: String,
}

pub struct AgentFactory {
    llm_factory: Arc<dyn LLMFactory>,
}

impl AgentFactory {
    pub fn new(llm_factory: Arc<dyn LLMFactory>) -> Self {
        Self { llm_factory }
    }

    pub fn create(
        &self,
        parameters: CreateAgentParameters,
    ) -> Result<Box<dyn Chain>, Box<dyn Error + Send>> {
        let llm: Box<dyn LLM> = self.llm_factory.create(CreateLLMParameters {
            model: parameters.model,
        })?;

        let agent = ConversationalAgentBuilder::new().build(llm).unwrap();

        let executor = AgentExecutor::from_agent(agent);

        Ok(Box::new(executor))
    }
}
