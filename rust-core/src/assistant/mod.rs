use std::sync::Arc;

use domain::{
    message::{
        message_status_mutator::MessageStatusMutator, MessageDeltaUpdateService, MessageRepository,
    },
    run::{run_status_mutator::RunStatusMutator, RunFactory, RunRepository},
    stream_thread_run_service::StreamThreadRunService,
    thread::{ThreadMessageFactory, ThreadRepository},
    thread_chat_completions_inference::ThreadChatCompletionInference,
};
use infrastructure::{SeaOrmMessageRepository, SeaOrmRunRepository, SeaOrmThreadRepository};

use crate::chat_completion::ChatCompletionDIModule;

pub mod domain;
pub mod infrastructure;

pub struct AgentDIModule {
    chat_completion_module: Arc<ChatCompletionDIModule>,
    connection: Arc<::sea_orm::DatabaseConnection>,
}

impl AgentDIModule {
    pub fn new(
        connection: Arc<::sea_orm::DatabaseConnection>,
        chat_completion_module: Arc<ChatCompletionDIModule>,
    ) -> Self {
        Self {
            chat_completion_module,
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

    pub fn get_thread_inference_service(&self) -> Arc<ThreadChatCompletionInference> {
        let chat_completion_inference_service = self.chat_completion_module.get_inference_factory();

        Arc::new(ThreadChatCompletionInference::new(
            chat_completion_inference_service,
        ))
    }

    pub fn get_run_status_mutator(&self) -> Arc<RunStatusMutator> {
        let run_repository = self.get_run_repository();

        Arc::new(RunStatusMutator::new(Arc::clone(&run_repository)))
    }

    pub fn get_run_message_status_mutator(&self) -> Arc<MessageStatusMutator> {
        let message_repository = self.get_message_repository();

        Arc::new(MessageStatusMutator::new(Arc::clone(&message_repository)))
    }

    pub fn get_stream_run_service(&self) -> Arc<StreamThreadRunService> {
        let run_factory = self.get_run_factory();
        let interence_service = self.get_thread_inference_service();
        let thread_repository = self.get_thread_repository();
        let message_repository = self.get_message_repository();
        let thread_message_factory =
            Arc::new(ThreadMessageFactory::new(Arc::clone(&message_repository)));
        let message_delta_update_service = Arc::new(MessageDeltaUpdateService::new(Arc::clone(
            &message_repository,
        )));
        let run_status_mutator = self.get_run_status_mutator();
        let message_status_mutator = self.get_run_message_status_mutator();

        Arc::new(StreamThreadRunService::new(
            run_factory,
            interence_service,
            thread_repository,
            thread_message_factory,
            message_delta_update_service,
            run_status_mutator,
            message_status_mutator,
        ))
    }
}
