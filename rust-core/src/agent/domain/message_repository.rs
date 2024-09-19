use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::entities::message;

#[derive(Default, Serialize, Deserialize)]
pub struct NewMessageModel {
    pub content: String,
    pub role: String,
    pub thread_id: Option<i32>,
    pub run_id: Option<i32>,
    pub attachments: Option<String>,
    pub metadata: Option<String>,
}

#[async_trait::async_trait]
pub trait MessageRepository: Sync + Send {
    async fn create(&self, message: NewMessageModel) -> Result<message::Model, Box<dyn Error>>;
    async fn find(&self, id: i32) -> Result<Option<message::Model>, Box<dyn Error>>;
    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
