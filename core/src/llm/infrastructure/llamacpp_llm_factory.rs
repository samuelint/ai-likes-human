use langchain_rust::language_models::llm::LLM;
use std::error::Error;

pub use crate::llm::domain::llm_factory::{CreateLLMParameters, LLMFactory};

const TMP_LOCAL_MODEL_PATH: &str = "~/.cache/lm-studio/models/lmstudio-community/Meta-Llama-3-8B-Instruct-GGUF/Meta-Llama-3-8B-Instruct-Q4_K_M.gguf";

pub struct LlamaCppLLMFactory {}

impl LlamaCppLLMFactory {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl LLMFactory for LlamaCppLLMFactory {
    fn is_compatible(&self, model: &str) -> bool {
        model.to_lowercase().contains("local")
    }

    async fn create(
        &self,
        parameters: &CreateLLMParameters,
    ) -> Result<Box<dyn LLM>, Box<dyn Error + Send>> {
        let llm = LLamaCPP::new(LLamaCPPArgs {
            model_path: TMP_LOCAL_MODEL_PATH.to_string(),
            ..Default::default()
        });
        Ok(Box::new(llm))
    }
}
