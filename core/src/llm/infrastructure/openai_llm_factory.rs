#[cfg(test)]
#[path = "./openai_llm_factory_tests.rs"]
mod openai_llm_factory_tests;

use std::error::Error;

use anyhow::anyhow;
use langchain_rust::{language_models::llm::LLM, llm::OpenAI};

pub use crate::llm::domain::llm_factory::{CreateLLMParameters, LLMFactory};

pub struct OpenAILLMFactory {}

impl OpenAILLMFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl LLMFactory for OpenAILLMFactory {
    fn is_compatible(&self, model: &str) -> bool {
        model.to_lowercase().contains("openai")
    }

    fn create(
        &self,
        parameters: CreateLLMParameters,
    ) -> Result<Box<dyn LLM>, Box<dyn Error + Send>> {
        let split_vec: Vec<&str> = parameters.model.split(':').collect();

        let model = match split_vec.last() {
            Some(&model) => model,
            None => return Err(anyhow!("Invalid model format").into()),
        };

        let llm = OpenAI::default().with_model(model);
        Ok(Box::new(llm))
    }
}
