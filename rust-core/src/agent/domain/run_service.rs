use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};

use super::{
    run_repository::RunRepository, thread_repository::ThreadRepository, CreateRunParams,
    CreateThreadParams,
};
use crate::entities::run;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct CreateRunDto {
    pub assistant_id: Option<String>,
    pub model: String,
    pub instructions: Option<String>,
    pub temperature: Option<i32>,
    pub metadata: Option<String>,
    pub stream: Option<bool>,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct CreateThreadAndRunDto {
    pub assistant_id: Option<String>,
    pub thread: CreateThreadParams,
    pub model: String,
    pub instructions: Option<String>,
    pub metadata: Option<String>,
    pub temperature: Option<i32>,
    pub stream: Option<bool>,
}

impl From<CreateThreadAndRunDto> for CreateThreadParams {
    fn from(dto: CreateThreadAndRunDto) -> Self {
        dto.thread
    }
}

impl From<CreateThreadAndRunDto> for CreateRunDto {
    fn from(dto: CreateThreadAndRunDto) -> Self {
        CreateRunDto {
            assistant_id: dto.assistant_id,
            model: dto.model,
            instructions: dto.instructions,
            temperature: dto.temperature,
            metadata: dto.metadata,
            stream: dto.stream,
        }
    }
}

pub struct RunService {
    run_repository: Arc<dyn RunRepository>,
    thread_repository: Arc<dyn ThreadRepository>,
}

impl RunService {
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
        dto: CreateThreadAndRunDto,
    ) -> Result<run::Model, Box<dyn Error>> {
        let thread = self.thread_repository.create(dto.clone().into()).await?;

        self.create(thread.id, dto.into()).await
    }

    pub async fn create(
        &self,
        thread_id: i32,
        new_run: CreateRunDto,
    ) -> Result<run::Model, Box<dyn Error>> {
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
