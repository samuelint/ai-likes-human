use std::sync::Arc;

use domain::llm_factory::LLMFactory;
use infrastructure::{llm_factory_router::LLMFactoryRouter, openai_llm_factory::OpenAILLMFactory};

pub mod domain;
pub mod infrastructure;

pub struct LLMDIModule {}

impl LLMDIModule {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_llm_factory(&self) -> Arc<dyn LLMFactory> {
        let openai_llm_factory: Arc<dyn LLMFactory> = Arc::new(OpenAILLMFactory::new());

        Arc::new(LLMFactoryRouter::new(vec![openai_llm_factory]))
    }
}
