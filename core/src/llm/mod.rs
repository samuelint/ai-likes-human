use std::sync::Arc;

use domain::{
    agent::{
        base_agent_factory::BaseAgentFactory, default_agent_factory::DefaultAgentFactory,
        default_system_prompt_factory::DefaultSystemPromptFactory, AgentFactory,
    },
    api_key_service::{ApiKeyService, ApiKeyServiceImpl},
    llm_factory::LLMFactory,
};
use infrastructure::{
    anthropic_llm_factory::AnthropicLLMFactory, llm_factory_router::LLMFactoryRouter,
    openai_llm_factory::OpenAILLMFactory,
};

use crate::{configuration::ConfigurationDIModule, profile::ProfileDIModule};

pub mod domain;
pub mod infrastructure;

pub struct LLMDIModule {
    configuration_module: Arc<ConfigurationDIModule>,
    profile_module: Arc<ProfileDIModule>,
}

impl LLMDIModule {
    pub fn new(
        configuration_module: Arc<ConfigurationDIModule>,
        profile_module: Arc<ProfileDIModule>,
    ) -> Self {
        Self {
            configuration_module,
            profile_module,
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

    fn get_default_system_prompt_factory(&self) -> Arc<DefaultSystemPromptFactory> {
        let selected_profile_service = self.profile_module.get_selected_profiles_service();

        Arc::new(DefaultSystemPromptFactory::new(
            selected_profile_service.clone(),
        ))
    }

    fn get_default_agent_factory(&self) -> Arc<DefaultAgentFactory> {
        let llm_factory = self.get_llm_factory();
        let system_prompt_factory = self.get_default_system_prompt_factory();

        let default = DefaultAgentFactory::new(llm_factory, system_prompt_factory);

        Arc::new(default)
    }

    fn get_base_agent_factories(&self) -> Vec<Arc<dyn BaseAgentFactory>> {
        let default = self.get_default_agent_factory();

        vec![default]
    }

    pub fn get_agent_factory(&self) -> Arc<AgentFactory> {
        let base_factories = self.get_base_agent_factories();

        Arc::new(AgentFactory::new(base_factories))
    }
}
