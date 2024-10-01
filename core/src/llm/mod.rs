use std::sync::Arc;

use domain::{
    api_key_service::{ApiKeyService, ApiKeyServiceImpl},
    llm_factory::LLMFactory,
};
use infrastructure::{
    anthropic_llm_factory::AnthropicLLMFactory, llm_factory_router::LLMFactoryRouter,
    openai_llm_factory::OpenAILLMFactory,
};

use crate::configuration::ConfigurationDIModule;

pub mod domain;
pub mod infrastructure;

pub struct LLMDIModule {
    configuration_module: Arc<ConfigurationDIModule>,
}

impl LLMDIModule {
    pub fn new(configuration_module: Arc<ConfigurationDIModule>) -> Self {
        Self {
            configuration_module,
        }
    }

    pub fn get_api_key_service(&self) -> Arc<dyn ApiKeyService> {
        let configuration_repository = self.configuration_module.get_configuration_repository();

        Arc::new(ApiKeyServiceImpl::new(Arc::clone(
            &configuration_repository,
        )))
    }

    pub fn get_llm_factory(&self) -> Arc<dyn LLMFactory> {
        let api_key_service = self.get_api_key_service();
        let openai_llm_factory: Arc<dyn LLMFactory> =
            Arc::new(OpenAILLMFactory::new(Arc::clone(&api_key_service)));
        let anthropic_llm_factory: Arc<dyn LLMFactory> =
            Arc::new(AnthropicLLMFactory::new(Arc::clone(&api_key_service)));

        Arc::new(LLMFactoryRouter::new(vec![
            openai_llm_factory,
            anthropic_llm_factory,
        ]))
    }
}
