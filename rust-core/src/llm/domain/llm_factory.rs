use langchain_rust::language_models::llm::LLM;
use mockall::predicate::*;
use mockall::*;
use std::error::Error;

pub struct CreateLLMParameters {
    pub model: String,
}

#[async_trait::async_trait]
#[automock]
pub trait LLMFactory: Send + Sync {
    fn is_compatible(&self, model: &str) -> bool;
    fn create(&self, parameters: CreateLLMParameters) -> Result<Box<dyn LLM>, Box<dyn Error>>;
}
