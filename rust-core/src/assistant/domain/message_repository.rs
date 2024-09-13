use std::error::Error;

use crate::entities::message;

pub struct NewMessageModel {
    pub content: String,
    pub role: String,
    pub attachments: Option<String>,
    pub metadata: Option<String>,
}

#[async_trait::async_trait]
pub trait MessageRepository: Sync + Send {
    async fn create(&self, message: NewMessageModel) -> Result<message::Model, Box<dyn Error>>;
    async fn find(&self, id: i32) -> Result<Option<message::Model>, Box<dyn Error>>;
    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
