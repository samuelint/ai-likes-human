use std::error::Error;

use serde::{Deserialize, Serialize};

use super::dto::ThreadDto;
use crate::utils::PageRequest;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct CreateThreadMessageParams {
    pub content: String,
    pub role: String,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct CreateThreadParams {
    pub metadata: Option<String>,
    pub messages: Vec<CreateThreadMessageParams>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct UpdateThreadParams {
    pub id: String,
    pub metadata: Option<String>,
}

#[async_trait::async_trait]
pub trait ThreadRepository: Sync + Send {
    async fn create(&self, thread: CreateThreadParams) -> Result<ThreadDto, Box<dyn Error + Send>>;
    async fn update(&self, thread: UpdateThreadParams) -> Result<ThreadDto, Box<dyn Error>>;
    async fn list_by_page(&self, args: PageRequest) -> Result<Vec<ThreadDto>, Box<dyn Error>>;
    async fn find(&self, id: String) -> Result<Option<ThreadDto>, Box<dyn Error>>;
    async fn delete(&self, id: String) -> Result<(), Box<dyn Error>>;
}
