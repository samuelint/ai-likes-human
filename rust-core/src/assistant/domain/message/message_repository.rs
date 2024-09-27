use std::error::Error;

use mockall::*;

use crate::assistant::domain::dto::{
    DbCreateThreadMessageDto, DbUpdateThreadMessageDto, ThreadMessageDto,
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
    async fn find_by_thread_id(&self, id: String) -> Result<Vec<ThreadMessageDto>, Box<dyn Error>>;

    async fn delete(&self, id: String) -> Result<(), Box<dyn Error>>;
}
