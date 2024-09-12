use langchain_rust::language_models::llm::LLM;
use std::error::Error;

pub struct CreateLLMParameters {
    pub model: String,
}

pub trait LLMFactory<T>
where
    T: LLM,
{
    fn is_compatible(&self, model: &str) -> bool;
    fn create(&self, parameters: CreateLLMParameters) -> Result<T, Box<dyn Error>>;
}
