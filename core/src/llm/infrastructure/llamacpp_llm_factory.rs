use async_llama_cpp::ModelOptions;
use langchain_rust::language_models::llm::LLM;
use std::{error::Error, sync::Arc};

pub use crate::llm::domain::llm_factory::{CreateLLMParameters, LLMFactory};

use super::{llamacpp_llm::LLamaCppLLM, tmp_model::get_tmp_model_path};

const TMP_LOCAL_MODEL_PATH: &str = "~/.cache/lm-studio/models/lmstudio-community/Meta-Llama-3-8B-Instruct-GGUF/Meta-Llama-3-8B-Instruct-Q4_K_M.gguf";

pub struct LlamaCppLLMFactory {
    llamacpp_builder: Arc<async_llama_cpp::Builder>,
}

impl LlamaCppLLMFactory {
    pub fn new(llamacpp_builder: Arc<async_llama_cpp::Builder>) -> Self {
        Self { llamacpp_builder }
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
        let model = parameters.model.clone();
        let model_path = get_tmp_model_path();
        let llamacpp = self.llamacpp_builder.build();

        let llm = LLamaCppLLM::new(llamacpp, model);

        Ok(Box::new(llm))
    }
}
