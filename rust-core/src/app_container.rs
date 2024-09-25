use std::{error::Error, sync::Arc};

use crate::{
    assistant::AgentDIModule, app_configuration::AppConfiguration,
    chat_completion::ChatCompletionDIModule, configuration::ConfigurationDIModule,
    infrastructure::sea_orm::ConnectionFactory, llm::LLMDIModule,
};

pub struct AppContainer {
    pub config: AppConfiguration,
    pub sea_orm_connection: Arc<::sea_orm::DatabaseConnection>,
    pub configuration_module: ConfigurationDIModule,
    pub llm_module: Arc<LLMDIModule>,
    pub chat_completion_module: ChatCompletionDIModule,
    pub agent_module: AgentDIModule,
}

impl AppContainer {
    pub async fn new(config: AppConfiguration) -> Result<Self, Box<dyn Error>> {
        let connection_factory = ConnectionFactory::new(config.database_url.clone());
        let connection: Arc<::sea_orm::DatabaseConnection> = connection_factory.create().await?;

        let configuration_module = ConfigurationDIModule::new(Arc::clone(&connection));
        let llm_module = Arc::new(LLMDIModule::new());
        let chat_completion_module = ChatCompletionDIModule::new(Arc::clone(&llm_module));
        let agent_module: AgentDIModule =
            AgentDIModule::new(Arc::clone(&connection), Arc::clone(&llm_module));

        Ok(Self {
            config,
            sea_orm_connection: connection,
            configuration_module,
            llm_module,
            chat_completion_module,
            agent_module,
        })
    }

    pub async fn new_in_memory() -> Result<Self, Box<dyn Error>> {
        let config = AppConfiguration {
            database_url: "sqlite::memory:".to_string(),
            ..AppConfiguration::default()
        };
        Self::new(config).await
    }
}
