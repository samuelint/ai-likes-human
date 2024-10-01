#[cfg(test)]
#[path = "./anthropic_llm_factory_tests.rs"]
mod anthropic_llm_factory_tests;

use std::{error::Error, sync::Arc};

use anyhow::anyhow;
use langchain_rust::{
    language_models::{llm::LLM, options::CallOptions},
    llm::Claude,
};

use crate::llm::domain::api_key_service::ApiKeyService;
pub use crate::llm::domain::llm_factory::{CreateLLMParameters, LLMFactory};

pub struct AnthropicLLMFactory {
    api_key_service: Arc<dyn ApiKeyService>,
}

impl AnthropicLLMFactory {
    pub fn new(api_key_service: Arc<dyn ApiKeyService>) -> Self {
        Self { api_key_service }
    }
}

#[async_trait::async_trait]
impl LLMFactory for AnthropicLLMFactory {
    fn is_compatible(&self, model: &str) -> bool {
        model.to_lowercase().contains("anthropic")
    }

    async fn create(
        &self,
        parameters: &CreateLLMParameters,
    ) -> Result<Box<dyn LLM>, Box<dyn Error + Send>> {
        let split_vec: Vec<&str> = parameters.model.split(':').collect();

        let model = match split_vec.last() {
            Some(&model) => model,
            None => return Err(anyhow!("Invalid model format").into()),
        };
        let api_key = self
            .api_key_service
            .get_api_key("ANTHROPIC_API_KEY")
            .await?;

        let llm = Claude::default()
            .with_model(model)
            .with_api_key(api_key)
            .with_options(CallOptions {
                temperature: parameters.temperature,
                ..CallOptions::default()
            });
        Ok(Box::new(llm))
    }
}
