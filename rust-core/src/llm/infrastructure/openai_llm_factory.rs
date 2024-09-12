#[cfg(test)]
#[path = "./openai_llm_factory_tests.rs"]
mod openai_llm_factory_tests;

use std::error::Error;

use langchain_rust::llm::{OpenAI, OpenAIConfig};

pub use crate::llm::domain::llm_factory::{CreateLLMParameters, LLMFactory};

pub struct OpenAILLMFactory {}

impl LLMFactory<OpenAI<OpenAIConfig>> for OpenAILLMFactory {
    fn is_compatible(&self, model: &str) -> bool {
        model.to_lowercase().contains("openai")
    }

    fn create(
        &self,
        parameters: CreateLLMParameters,
    ) -> Result<OpenAI<OpenAIConfig>, Box<dyn Error>> {
        let split_vec: Vec<&str> = parameters.model.split(':').collect();

        let model = match split_vec.last() {
            Some(&model) => model,
            None => return Err("Model is not valid".into()),
        };

        Ok(OpenAI::default().with_model(model))
    }
}
