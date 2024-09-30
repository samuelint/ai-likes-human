use std::error::Error;

use mockall::*;

use crate::assistant::domain::dto::{
    DbCreateThreadMessageDto, DbUpdateThreadMessageDto, PageRequest, PageResponse, ThreadMessageDto,
};

#[async_trait::async_trait]
#[automock]
pub trait MessageRepository: Sync + Send {
    async fn create(
        &self,
        message: DbCreateThreadMessageDto,
    ) -> Result<ThreadMessageDto, Box<dyn Error + Send>>;
    async fn create_many(
        &self,
        messages: Vec<DbCreateThreadMessageDto>,
    ) -> Result<(), Box<dyn Error + Send>>;

    async fn find(&self, id: String) -> Result<Option<ThreadMessageDto>, Box<dyn Error>>;
    async fn update(
        &self,
        id: &str,
        message: &DbUpdateThreadMessageDto,
    ) -> Result<ThreadMessageDto, Box<dyn Error + Send>>;
    async fn list_by_thread_id_paginated(
        &self,
        thread_id: &str,
        page: &PageRequest,
    ) -> Result<PageResponse<ThreadMessageDto>, Box<dyn Error + Send>>;

    async fn delete(&self, id: String) -> Result<(), Box<dyn Error>>;
}
