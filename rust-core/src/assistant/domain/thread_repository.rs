use std::error::Error;

use mockall::automock;
use serde::{Deserialize, Serialize};

use super::dto::{CreateThreadMessageDto, ThreadDto, ThreadMessageDto};
use crate::utils::PageRequest;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct CreateThreadParams {
    pub metadata: Option<String>,
    pub messages: Vec<CreateThreadMessageDto>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct UpdateThreadParams {
    pub id: String,
    pub metadata: Option<String>,
}

#[async_trait::async_trait]
#[automock]
pub trait ThreadRepository: Sync + Send {
    async fn create(&self, thread: CreateThreadParams) -> Result<ThreadDto, Box<dyn Error + Send>>;
    async fn update(&self, thread: UpdateThreadParams) -> Result<ThreadDto, Box<dyn Error>>;
    async fn list_by_page(&self, args: PageRequest) -> Result<Vec<ThreadDto>, Box<dyn Error>>;
    async fn find(&self, id: &str) -> Result<Option<ThreadDto>, Box<dyn Error>>;
    async fn find_messages(&self, id: &str)
        -> Result<Vec<ThreadMessageDto>, Box<dyn Error + Send>>;
    async fn delete(&self, id: &str) -> Result<(), Box<dyn Error>>;
}
