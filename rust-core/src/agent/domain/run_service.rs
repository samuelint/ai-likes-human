use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};

use super::{run_repository::RunRepository, CreateRunParams};
use crate::entities::run;

pub struct RunService {
    run_repository: Arc<dyn RunRepository>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct CreateThreadRunDto {
    pub assistant_id: String,
    pub model: String,
    pub instructions: Option<String>,
    pub temperature: Option<i32>,
    pub metadata: Option<String>,
}

impl RunService {
    pub fn new(run_repository: Arc<dyn RunRepository>) -> Self {
        Self { run_repository }
    }

    pub async fn create(
        &self,
        thread_id: i32,
        new_run: CreateThreadRunDto,
    ) -> Result<run::Model, Box<dyn Error>> {
        let run = self
            .run_repository
            .create(CreateRunParams {
                assistant_id: new_run.assistant_id,
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
