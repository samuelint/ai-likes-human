use std::{error::Error, sync::Arc};

use crate::assistant::domain::{
    dto::{ApiCreateRunDto, ApiCreateThreadAndRunDto, DbCreateRunDto, RunDto, ThreadDto},
    thread_repository::ThreadRepository,
};

use super::RunRepository;

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

    pub async fn create_thread_and_run(
        &self,
        dto: &ApiCreateThreadAndRunDto,
    ) -> Result<(ThreadDto, RunDto), Box<dyn std::error::Error + Send>> {
        let create_run_dto: ApiCreateRunDto = dto.into();
        let thread = self.thread_repository.create(dto.into()).await?;
        let run = self.create_run(&thread.id, &create_run_dto).await?;

        Ok((thread, run))
    }

    pub async fn create_run<'a>(
        &self,
        thread_id: &'a str,
        new_run: &ApiCreateRunDto,
    ) -> Result<RunDto, Box<dyn Error + Send>> {
        let new_run = new_run.clone();
        let assistant_id = new_run.assistant_id.unwrap_or("default".to_string());
        let run = self
            .run_repository
            .create(DbCreateRunDto {
                assistant_id,
                thread_id: thread_id.to_string(),
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
