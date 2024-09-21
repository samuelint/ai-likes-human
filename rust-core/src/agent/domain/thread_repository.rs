use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::{entities::thread, utils::PageRequest};

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
    pub id: i32,
    pub metadata: Option<String>,
}

#[async_trait::async_trait]
pub trait ThreadRepository: Sync + Send {
    async fn create(&self, thread: CreateThreadParams) -> Result<thread::Model, Box<dyn Error>>;
    async fn update(&self, thread: UpdateThreadParams) -> Result<thread::Model, Box<dyn Error>>;
    async fn list_by_page(&self, args: PageRequest) -> Result<Vec<thread::Model>, Box<dyn Error>>;
    async fn find(&self, id: i32) -> Result<Option<thread::Model>, Box<dyn Error>>;
    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
