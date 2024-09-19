use std::sync::Arc;

use domain::agent_factory::AgentFactoryImpl;

use crate::{llm::{domain::llm_factory::LLMFactory, LLMDIModule}, AgentFactory};

pub mod app;
pub mod domain;
pub mod infrastructure;


pub struct AgentDIModule {
    llm_module: LLMDIModule,
}

impl AgentDIModule {
    pub fn new(llm_module: LLMDIModule) -> Self {
        Self { llm_module }
    }

    pub fn get_agent_factory(&self) -> Arc<dyn AgentFactory> {
        let llm_factory: Arc<dyn LLMFactory> = self.llm_module.get_llm_factory();

        Arc::new(AgentFactoryImpl::new(llm_factory))
    }
}
