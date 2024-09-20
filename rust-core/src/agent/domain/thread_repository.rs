use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::{entities::thread, utils::PageRequest};

use super::message_repository::CreateMessageDto;

#[derive(Default, Serialize, Deserialize)]
pub struct CreateThreadDto {
    pub metadata: Option<String>,
    pub messages: Vec<CreateMessageDto>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct UpdateThreadDto {
    pub id: i32,
    pub metadata: Option<String>,
}

#[async_trait::async_trait]
pub trait ThreadRepository: Sync + Send {
    async fn create(&self, thread: CreateThreadDto) -> Result<thread::Model, Box<dyn Error>>;
    async fn update(&self, thread: UpdateThreadDto) -> Result<thread::Model, Box<dyn Error>>;
    async fn list_by_page(&self, args: PageRequest) -> Result<Vec<thread::Model>, Box<dyn Error>>;
    async fn find(&self, id: i32) -> Result<Option<thread::Model>, Box<dyn Error>>;
    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
