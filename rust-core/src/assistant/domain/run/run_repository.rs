use std::error::Error;

use mockall::automock;

use crate::assistant::domain::dto::{
    DbCreateRunDto, DbUpdateRunDto, PageRequest, PageResponse, RunDto,
};

#[async_trait::async_trait]
#[automock]
pub trait RunRepository: Sync + Send {
    async fn create(&self, new_run: DbCreateRunDto) -> Result<RunDto, Box<dyn Error + Send>>;
    async fn list_by_thread_paginated(
        &self,
        thread_id: &str,
        page: PageRequest,
    ) -> Result<PageResponse<RunDto>, Box<dyn Error>>;
    async fn find(&self, id: &str) -> Result<Option<RunDto>, Box<dyn Error>>;
    async fn update(&self, id: &str, dto: &DbUpdateRunDto)
        -> Result<RunDto, Box<dyn Error + Send>>;
    async fn delete(&self, id: &str) -> Result<(), Box<dyn Error>>;
}
