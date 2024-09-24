use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::utils::PageRequest;

use super::dto::RunDto;

#[derive(Default, Serialize, Deserialize)]
pub struct CreateRunParams {
    pub assistant_id: String,
    pub thread_id: String,
    pub model: String,
    pub status: String,
    pub instructions: Option<String>,
    pub temperature: Option<i32>,
    pub metadata: Option<String>,
}

#[async_trait::async_trait]
pub trait RunRepository: Sync + Send {
    async fn create(&self, message: CreateRunParams) -> Result<RunDto, Box<dyn Error + Send>>;
    async fn list_by_thread_paginated(
        &self,
        thread_id: String,
        page: PageRequest,
    ) -> Result<Vec<RunDto>, Box<dyn Error>>;
    async fn find(&self, id: String) -> Result<Option<RunDto>, Box<dyn Error>>;
    async fn delete(&self, id: String) -> Result<(), Box<dyn Error>>;
}
