use std::{error::Error, sync::Arc};

use langchain_rust::{
    agent::{AgentExecutor, ConversationalAgentBuilder},
    chain::{options::ChainCallOptions, Chain},
};

use crate::llm::domain::llm_factory::{CreateLLMParameters, LLMFactory};

use super::{
    base_agent_factory::{BaseAgentFactory, CreateAgentArgs},
    default_system_prompt_factory::DefaultSystemPromptFactory,
};

pub struct DefaultAgentFactory {
    llm_factory: Arc<dyn LLMFactory>,
    system_prompt_factory: Arc<DefaultSystemPromptFactory>,
}

impl DefaultAgentFactory {
    pub fn new(
        llm_factory: Arc<dyn LLMFactory>,
        system_prompt_factory: Arc<DefaultSystemPromptFactory>,
    ) -> Self {
        Self {
            llm_factory,
            system_prompt_factory,
        }
    }

    fn create_options(args: &CreateAgentArgs) -> ChainCallOptions {
        let options: ChainCallOptions = ChainCallOptions::new();
        let options = match args.temperature {
            Some(temperature) => options.with_temperature(temperature),
            None => options,
        };

        options
    }
}

#[async_trait::async_trait]
impl BaseAgentFactory for DefaultAgentFactory {
    fn is_compatible(&self, _agent_id: &str) -> bool {
        true
    }

    async fn create(&self, args: CreateAgentArgs) -> Result<Box<dyn Chain>, Box<dyn Error + Send>> {
        let system_prompt = self.system_prompt_factory.create().await?;
        let llm = self
            .llm_factory
            .create(&CreateLLMParameters {
                model: args.model.to_string(),
                temperature: args.temperature,
                ..Default::default()
            })
            .await?;

        let agent = ConversationalAgentBuilder::new()
            // .tools(&[Arc::new(command_executor)])
            .options(Self::create_options(&args))
            .prefix(system_prompt)
            .build(llm)
            .unwrap();

            

        let executor = AgentExecutor::from_agent(agent);

        Ok(Box::new(executor))
    }
}
