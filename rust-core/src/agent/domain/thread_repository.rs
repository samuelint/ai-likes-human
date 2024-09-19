use std::error::Error;

use crate::entities::thread;

pub struct NewThreadModel {
    pub metadata: Option<String>,
}

pub struct ListByPage {
    pub after: Option<u64>,
    pub before: Option<u64>,
    pub limit: Option<u64>,
}

#[async_trait::async_trait]
pub trait ThreadRepository: Sync + Send {
    async fn create(&self, message: NewThreadModel) -> Result<thread::Model, Box<dyn Error>>;
    async fn list_by_page(&self, args: ListByPage) -> Result<Vec<thread::Model>, Box<dyn Error>>;
    async fn find(&self, id: i32) -> Result<Option<thread::Model>, Box<dyn Error>>;
    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
