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

    pub fn get_llm_client<'a>(&self, model: &'a str) -> Result<Box<dyn LLM>, Box<dyn Error + Send>> {
        let factory = self.container.llm_module.get_llm_factory();

        factory.create(CreateLLMParameters { model: model.to_string() })
    }
}
