use std::error::Error;

use crate::entities::run;

pub struct NewRunModel {
    pub assistant_id: String,
    pub thread_id: i32,
    pub model: String,
    pub status: String,
    pub instructions: Option<String>,
    pub temperature: Option<i32>,
    pub metadata: Option<String>,
}

pub struct ListByPage {
    pub after: Option<u64>,
    pub before: Option<u64>,
    pub limit: Option<u64>,
}

#[async_trait::async_trait]
pub trait RunRepository: Sync + Send {
    async fn create(&self, message: NewRunModel) -> Result<run::Model, Box<dyn Error>>;
    async fn list_by_page(&self, args: ListByPage) -> Result<Vec<run::Model>, Box<dyn Error>>;
    async fn find(&self, id: i32) -> Result<Option<run::Model>, Box<dyn Error>>;
    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
