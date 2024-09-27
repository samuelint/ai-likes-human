use std::error::Error;

use mockall::automock;

use crate::{
    assistant::domain::dto::{DbCreateThreadDto, DbUpdateThreadDto, ThreadDto, ThreadMessageDto},
    utils::PageRequest,
};

#[async_trait::async_trait]
#[automock]
pub trait ThreadRepository: Sync + Send {
    async fn create(&self, thread: DbCreateThreadDto) -> Result<ThreadDto, Box<dyn Error + Send>>;
    async fn update(&self, thread: DbUpdateThreadDto) -> Result<ThreadDto, Box<dyn Error>>;
    async fn list_by_page(&self, args: PageRequest) -> Result<Vec<ThreadDto>, Box<dyn Error>>;
    async fn find(&self, id: &str) -> Result<Option<ThreadDto>, Box<dyn Error>>;
    async fn find_messages(&self, id: &str)
        -> Result<Vec<ThreadMessageDto>, Box<dyn Error + Send>>;
    async fn delete(&self, id: &str) -> Result<(), Box<dyn Error>>;
}
