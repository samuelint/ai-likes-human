use std::{error::Error, sync::Arc};

use super::{
    dto::{CreateRunDto, CreateThreadAndRunDto},
    run_repository::RunRepository,
    thread_repository::ThreadRepository,
    CreateRunParams,
};
use crate::entities::{run, thread};

pub struct RunFactory {
    run_repository: Arc<dyn RunRepository>,
    thread_repository: Arc<dyn ThreadRepository>,
}

impl RunFactory {
    pub fn new(
        run_repository: Arc<dyn RunRepository>,
        thread_repository: Arc<dyn ThreadRepository>,
    ) -> Self {
        Self {
            run_repository,
            thread_repository,
        }
    }

    pub async fn create_thread_and_run<'a>(
        &self,
        dto: &CreateThreadAndRunDto,
    ) -> Result<(thread::Model, run::Model), Box<dyn std::error::Error + Send>> {
        let thread = self.thread_repository.create(dto.clone().into()).await?;
        let run = self.create_run(thread.id, dto.clone().into()).await?;

        Ok((thread, run))
    }

    pub async fn create_run(
        &self,
        thread_id: i32,
        new_run: CreateRunDto,
    ) -> Result<run::Model, Box<dyn Error + Send>> {
        let assistant_id = new_run.assistant_id.unwrap_or("default".to_string());
        let run = self
            .run_repository
            .create(CreateRunParams {
                assistant_id,
                thread_id,
                model: new_run.model,
                status: "queued".to_string(),
                instructions: None,
                temperature: new_run.temperature,
                metadata: None,
            })
            .await?;

        Ok(run)
    }
}
