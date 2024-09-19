use std::{error::Error, sync::Arc};

use langchain_rust::language_models::llm::LLM;

use crate::{llm::domain::llm_factory::CreateLLMParameters, AppContainer};

pub struct ApiFacade {
    container: Arc<AppContainer>,
}

impl ApiFacade {
    pub fn new(container: Arc<AppContainer>) -> Self {
        Self { container }
    }

    pub fn get_llm_client(&self, model: String) -> Result<Box<dyn LLM>, Box<dyn Error>> {
        let factory = self.container.llm_module.get_llm_factory();

        factory.create(CreateLLMParameters { model })
    }
}
