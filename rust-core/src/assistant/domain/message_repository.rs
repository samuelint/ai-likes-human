use std::error::Error;

use mockall::*;

use super::dto::{CreateThreadMessageDto, ThreadMessageDto, UpdateThreadMessageDto};

#[async_trait::async_trait]
#[automock]
pub trait MessageRepository: Sync + Send {
    async fn create(
        &self,
        message: CreateThreadMessageDto,
    ) -> Result<ThreadMessageDto, Box<dyn Error + Send>>;
    async fn create_many(
        &self,
        messages: Vec<CreateThreadMessageDto>,
    ) -> Result<(), Box<dyn Error + Send>>;

    async fn find(&self, id: String) -> Result<Option<ThreadMessageDto>, Box<dyn Error>>;
    async fn update(
        &self,
        message: UpdateThreadMessageDto,
    ) -> Result<ThreadMessageDto, Box<dyn Error + Send>>;
    async fn find_by_thread_id(&self, id: String) -> Result<Vec<ThreadMessageDto>, Box<dyn Error>>;

    async fn delete(&self, id: String) -> Result<(), Box<dyn Error>>;
}
