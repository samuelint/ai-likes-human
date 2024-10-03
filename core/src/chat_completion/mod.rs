mod domain;

pub use domain::*;
use inference::Inference;
use llm_inference::LLMInference;
use profiled_inference_factory::ProfiledInferenceFactory;
use std::sync::Arc;

use crate::{llm::LLMDIModule, profile::ProfileDIModule};

pub struct ChatCompletionDIModule {
    llm_module: Arc<LLMDIModule>,
    profile_module: Arc<ProfileDIModule>,
}

impl ChatCompletionDIModule {
    pub fn new(llm_module: Arc<LLMDIModule>, profile_module: Arc<ProfileDIModule>) -> Self {
        Self {
            llm_module,
            profile_module,
        }
    }

    pub fn get_llm_inference(&self) -> Arc<LLMInference> {
        let llm_factory = self.llm_module.get_llm_factory();

        Arc::new(LLMInference::new(llm_factory))
    }

    pub fn get_inference(&self) -> Arc<dyn Inference> {
        let llm_inference: Arc<LLMInference> = self.get_llm_inference();
        let system_prompt_factory = self.profile_module.get_profile_system_prompt_factory();

        Arc::new(ProfiledInferenceFactory::new(
            llm_inference,
            system_prompt_factory,
        ))
    }
}
