use std::sync::Arc;

use domain::{
    agent_factory::AgentFactory, message_repository::MessageRepository, run_factory::RunFactory,
    run_repository::RunRepository, stream_run_service::StreamRunService,
    thread_repository::ThreadRepository,
};
use infrastructure::{SeaOrmMessageRepository, SeaOrmRunRepository, SeaOrmThreadRepository};

use crate::llm::{domain::llm_factory::LLMFactory, LLMDIModule};

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

    fn get_sea_orm_run_repository(&self) -> Arc<SeaOrmRunRepository> {
        let connection = self.get_connection();
        Arc::new(SeaOrmRunRepository::new(Arc::clone(&connection)))
    }

    fn get_sea_orm_message_repository(&self) -> Arc<SeaOrmMessageRepository> {
        let connection = self.get_connection();
        Arc::new(SeaOrmMessageRepository::new(Arc::clone(&connection)))
    }

    pub fn get_message_repository(&self) -> Arc<dyn MessageRepository> {
        self.get_sea_orm_message_repository()
    }

    pub fn get_run_repository(&self) -> Arc<dyn RunRepository> {
        self.get_sea_orm_run_repository()
    }

    pub fn get_thread_repository(&self) -> Arc<dyn ThreadRepository> {
        let connection = self.get_connection();
        let message_repository = self.get_sea_orm_message_repository();
        let run_repository = self.get_sea_orm_run_repository();

        Arc::new(SeaOrmThreadRepository::new(
            Arc::clone(&connection),
            message_repository,
            run_repository,
        ))
    }

    pub fn get_run_factory(&self) -> Arc<RunFactory> {
        let run_repository = self.get_run_repository();
        let thread_repository = self.get_thread_repository();

        Arc::new(RunFactory::new(run_repository, thread_repository))
    }

    pub fn get_stream_run_service(&self) -> Arc<StreamRunService> {
        let run_factory = self.get_run_factory();
        let llm_factory = self.llm_module.get_llm_factory();

        Arc::new(StreamRunService::new(run_factory, llm_factory))
    }

    pub fn get_agent_factory(&self) -> Arc<AgentFactory> {
        let llm_factory: Arc<dyn LLMFactory> = self.llm_module.get_llm_factory();

        Arc::new(AgentFactory::new(llm_factory))
    }
}
