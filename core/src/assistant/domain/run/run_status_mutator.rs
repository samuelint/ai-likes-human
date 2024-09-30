#[cfg(test)]
#[path = "./run_status_mutator_test.rs"]
mod run_status_mutator_test;

use std::sync::Arc;

use crate::assistant::domain::dto::{DbUpdateRunDto, RunDto};

use super::RunRepository;

pub struct RunStatusMutator {
    run_repository: Arc<dyn RunRepository>,
}

impl RunStatusMutator {
    pub fn new(run_repository: Arc<dyn RunRepository>) -> Self {
        Self { run_repository }
    }

    pub async fn mutate(
        &self,
        run: &RunDto,
        status: &str,
    ) -> Result<RunDto, Box<dyn std::error::Error + Send>> {
        self.run_repository
            .update(
                &run.id,
                &DbUpdateRunDto {
                    status: Some(status.to_string()),
                    ..DbUpdateRunDto::default()
                },
            )
            .await
    }

    pub async fn mutate_to_completed(
        &self,
        run: &RunDto,
    ) -> Result<RunDto, Box<dyn std::error::Error + Send>> {
        self.mutate(run, "completed").await
    }

    pub async fn mutate_to_in_progress(
        &self,
        run: &RunDto,
    ) -> Result<RunDto, Box<dyn std::error::Error + Send>> {
        self.mutate(run, "in-progress").await
    }
}
