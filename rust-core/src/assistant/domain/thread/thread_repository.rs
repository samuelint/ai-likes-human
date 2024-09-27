use std::error::Error;

use mockall::automock;

use crate::assistant::domain::dto::{
    DbCreateThreadDto, DbUpdateThreadDto, PageRequest, PageResponse, ThreadDto,
};

#[async_trait::async_trait]
#[automock]
pub trait ThreadRepository: Sync + Send {
    async fn create(&self, thread: DbCreateThreadDto) -> Result<ThreadDto, Box<dyn Error + Send>>;
    async fn update(&self, thread: DbUpdateThreadDto) -> Result<ThreadDto, Box<dyn Error>>;
    async fn list_by_page(
        &self,
        args: PageRequest,
    ) -> Result<PageResponse<ThreadDto>, Box<dyn Error>>;
    async fn find(&self, id: &str) -> Result<Option<ThreadDto>, Box<dyn Error>>;
    async fn delete(&self, id: &str) -> Result<(), Box<dyn Error>>;
}
