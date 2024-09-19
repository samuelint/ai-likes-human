use std::sync::Arc;

use domain::{
    agent_factory::AgentFactoryImpl, message_repository::MessageRepository,
    run_repository::RunRepository, thread_repository::ThreadRepository,
};
use infrastructure::{SeaOrmMessageRepository, SeaOrmRunRepository, SeaOrmThreadRepository};

use crate::{
    llm::{domain::llm_factory::LLMFactory, LLMDIModule},
    AgentFactory,
};

pub mod app;
pub mod domain;
pub mod infrastructure;

pub struct AgentDIModule {
    llm_module: LLMDIModule,
    connection: Arc<::sea_orm::DatabaseConnection>,
}

impl AgentDIModule {
    pub fn new(connection: Arc<::sea_orm::DatabaseConnection>, llm_module: LLMDIModule) -> Self {
        Self {
            llm_module,
            connection,
        }
    }

    fn get_connection(&self) -> Arc<::sea_orm::DatabaseConnection> {
        Arc::clone(&self.connection)
    }

    pub fn get_message_repository(&self) -> Arc<dyn MessageRepository> {
        let connection = self.get_connection();
        Arc::new(SeaOrmMessageRepository::new(Arc::clone(&connection)))
    }

    pub fn get_run_repository(&self) -> Arc<dyn RunRepository> {
        let connection = self.get_connection();
        Arc::new(SeaOrmRunRepository::new(Arc::clone(&connection)))
    }

    pub fn get_thread_repository(&self) -> Arc<dyn ThreadRepository> {
        let connection = self.get_connection();
        Arc::new(SeaOrmThreadRepository::new(Arc::clone(&connection)))
    }

    pub fn get_agent_factory(&self) -> Arc<dyn AgentFactory> {
        let llm_factory: Arc<dyn LLMFactory> = self.llm_module.get_llm_factory();

        Arc::new(AgentFactoryImpl::new(llm_factory))
    }
}
