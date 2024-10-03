mod domain;

pub use domain::*;
use llm_inference::LLMInference;
use std::sync::Arc;

use crate::llm::LLMDIModule;

pub struct ChatCompletionDIModule {
    llm_module: Arc<LLMDIModule>,
}

impl ChatCompletionDIModule {
    pub fn new(llm_module: Arc<LLMDIModule>) -> Self {
        Self { llm_module }
    }

    pub fn get_llm_inference(&self) -> Arc<LLMInference> {
        let llm_factory = self.llm_module.get_llm_factory();

        Arc::new(LLMInference::new(llm_factory))
    }
}
