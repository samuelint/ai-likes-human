use langchain_rust::language_models::llm::LLM;
use mockall::predicate::*;
use mockall::*;
use std::error::Error;

#[derive(Default)]
pub struct CreateLLMParameters {
    pub model: String,
    pub temperature: Option<f32>,
}

#[async_trait::async_trait]
#[automock]
pub trait LLMFactory: Send + Sync {
    fn is_compatible(&self, model: &str) -> bool;
    async fn create(
        &self,
        parameters: &CreateLLMParameters,
    ) -> Result<Box<dyn LLM>, Box<dyn Error + Send>>;
}
