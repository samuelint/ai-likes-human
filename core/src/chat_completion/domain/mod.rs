mod dto;
mod inference_service;
mod types;

use std::sync::Arc;

pub use dto::*;
pub use inference_service::InferenceService;
pub use types::*;

use crate::llm::LLMDIModule;

pub struct ChatCompletionDIModule {
    llm_module: Arc<LLMDIModule>,
}

impl ChatCompletionDIModule {
    pub fn new(llm_module: Arc<LLMDIModule>) -> Self {
        Self { llm_module }
    }

    pub fn get_inference_factory(&self) -> Arc<InferenceService> {
        let llm_factory = self.llm_module.get_llm_factory();

        Arc::new(InferenceService::new(llm_factory))
    }
}
