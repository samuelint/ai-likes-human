use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::entities::message;

#[derive(Default, Serialize, Deserialize)]
pub struct CreateMessageParams {
    pub content: String,
    pub role: String,
    pub thread_id: Option<i32>,
    pub run_id: Option<i32>,
    pub attachments: Option<String>,
    pub metadata: Option<String>,
}

#[async_trait::async_trait]
pub trait MessageRepository: Sync + Send {
    async fn create(
        &self,
        message: CreateMessageParams,
    ) -> Result<message::Model, Box<dyn Error + Send>>;
    async fn create_many(
        &self,
        messages: Vec<CreateMessageParams>,
    ) -> Result<(), Box<dyn Error + Send>>;

    async fn find(&self, id: i32) -> Result<Option<message::Model>, Box<dyn Error>>;
    async fn find_by_thread_id(&self, id: i32) -> Result<Vec<message::Model>, Box<dyn Error>>;

    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
