use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::{entities::run, utils::PageRequest};

#[derive(Default, Serialize, Deserialize)]
pub struct CreateRunParams {
    pub assistant_id: String,
    pub thread_id: i32,
    pub model: String,
    pub status: String,
    pub instructions: Option<String>,
    pub temperature: Option<i32>,
    pub metadata: Option<String>,
}

#[async_trait::async_trait]
pub trait RunRepository: Sync + Send {
    async fn create(&self, message: CreateRunParams) -> Result<run::Model, Box<dyn Error + Send>>;
    async fn list_by_thread_paginated(
        &self,
        thread_id: i32,
        page: PageRequest,
    ) -> Result<Vec<run::Model>, Box<dyn Error>>;
    async fn find(&self, id: i32) -> Result<Option<run::Model>, Box<dyn Error>>;
    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
