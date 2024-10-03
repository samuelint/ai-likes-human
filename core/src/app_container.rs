use std::{error::Error, sync::Arc};

use crate::{
    app_configuration::CoreConfiguration, assistant::AgentDIModule,
    chat_completion::ChatCompletionDIModule, configuration::ConfigurationDIModule,
    infrastructure::sea_orm::ConnectionFactory, llm::LLMDIModule, profile::ProfileDIModule,
};

pub struct AppContainer {
    pub config: CoreConfiguration,
    pub sea_orm_connection: Arc<::sea_orm::DatabaseConnection>,
    pub configuration_module: Arc<ConfigurationDIModule>,
    pub llm_module: Arc<LLMDIModule>,
    pub chat_completion_module: Arc<ChatCompletionDIModule>,
    pub profile_module: Arc<ProfileDIModule>,
    pub agent_module: AgentDIModule,
}

impl AppContainer {
    pub async fn new(config: CoreConfiguration) -> Result<Self, Box<dyn Error>> {
        let connection_factory = ConnectionFactory::new(config.database_url.clone());
        let connection: Arc<::sea_orm::DatabaseConnection> = connection_factory.create().await?;

        let configuration_module = Arc::new(ConfigurationDIModule::new(Arc::clone(&connection)));
        let profile_module = Arc::new(ProfileDIModule::new(Arc::clone(&connection)));
        let llm_module = Arc::new(LLMDIModule::new(
            configuration_module.clone(),
            profile_module.clone(),
        ));
        let chat_completion_module = Arc::new(ChatCompletionDIModule::new(
            Arc::clone(&llm_module),
            Arc::clone(&profile_module),
        ));
        let agent_module: AgentDIModule =
            AgentDIModule::new(Arc::clone(&connection), Arc::clone(&chat_completion_module));

        Ok(Self {
            config,
            sea_orm_connection: connection,
            configuration_module,
            llm_module,
            chat_completion_module,
            profile_module,
            agent_module,
        })
    }

    pub async fn new_in_memory() -> Result<Self, Box<dyn Error>> {
        let config = CoreConfiguration {
            database_url: "sqlite::memory:".to_string(),
            ..CoreConfiguration::default()
        };
        Self::new(config).await
    }
}
