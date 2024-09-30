#[cfg(test)]
#[path = "./llm_factory_router_test.rs"]
mod llm_factory_router_test;
use anyhow::anyhow;
use langchain_rust::language_models::llm::LLM;
use std::{error::Error, sync::Arc};

pub use crate::llm::domain::llm_factory::{CreateLLMParameters, LLMFactory};

pub struct LLMFactoryRouter {
    llm_factories: Vec<Arc<dyn LLMFactory>>,
}

impl LLMFactoryRouter {
    pub fn new(llm_factories: Vec<Arc<dyn LLMFactory>>) -> Self {
        Self { llm_factories }
    }

    fn find_factory(&self, model: &str) -> Option<Arc<dyn LLMFactory>> {
        for factory in &self.llm_factories {
            if factory.is_compatible(model) {
                return Some(Arc::clone(factory));
            }
        }

        None
    }
}

impl LLMFactory for LLMFactoryRouter {
    fn is_compatible(&self, model: &str) -> bool {
        for factory in &self.llm_factories {
            if factory.is_compatible(model) {
                return true;
            }
        }

        false
    }

    fn create(
        &self,
        parameters: CreateLLMParameters,
    ) -> Result<Box<dyn LLM>, Box<dyn Error + Send>> {
        let factory = self.find_factory(&parameters.model);

        if let Some(factory) = factory {
            return factory.create(parameters);
        }

        Err(anyhow!("No corresponding LLM for {}", parameters.model).into())
    }
}
