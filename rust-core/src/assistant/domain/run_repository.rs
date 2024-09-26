use std::error::Error;

use crate::utils::PageRequest;

use super::dto::{DbCreateRunDto, RunDto};

#[async_trait::async_trait]
pub trait RunRepository: Sync + Send {
    async fn create(&self, new_run: DbCreateRunDto) -> Result<RunDto, Box<dyn Error + Send>>;
    async fn list_by_thread_paginated(
        &self,
        thread_id: String,
        page: PageRequest,
    ) -> Result<Vec<RunDto>, Box<dyn Error>>;
    async fn find(&self, id: String) -> Result<Option<RunDto>, Box<dyn Error>>;
    async fn delete(&self, id: String) -> Result<(), Box<dyn Error>>;
}
