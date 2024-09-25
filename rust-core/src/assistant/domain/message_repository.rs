use serde::{Deserialize, Serialize};
use std::error::Error;

use mockall::*;

use super::dto::{ThreadMessageDto, UpdateThreadMessageDto};

#[derive(Default, Serialize, Deserialize)]
pub struct CreateMessageParams {
    pub content: String,
    pub role: String,
    pub thread_id: Option<String>,
    pub run_id: Option<String>,
    pub attachments: Option<String>,
    pub metadata: Option<String>,
}

#[async_trait::async_trait]
#[automock]
pub trait MessageRepository: Sync + Send {
    async fn create(
        &self,
        message: CreateMessageParams,
    ) -> Result<ThreadMessageDto, Box<dyn Error + Send>>;
    async fn create_many(
        &self,
        messages: Vec<CreateMessageParams>,
    ) -> Result<(), Box<dyn Error + Send>>;

    async fn find(&self, id: String) -> Result<Option<ThreadMessageDto>, Box<dyn Error>>;
    async fn update(
        &self,
        message: UpdateThreadMessageDto,
    ) -> Result<ThreadMessageDto, Box<dyn Error + Send>>;
    async fn find_by_thread_id(&self, id: String) -> Result<Vec<ThreadMessageDto>, Box<dyn Error>>;

    async fn delete(&self, id: String) -> Result<(), Box<dyn Error>>;
}
