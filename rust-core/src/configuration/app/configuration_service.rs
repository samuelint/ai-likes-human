use std::error::Error;
use std::sync::Arc;

use crate::configuration::domain::{ConfigurationRepository, NewConfigurationModel};
use crate::entities::configuration;

#[async_trait::async_trait]
pub trait ConfigurationService: Send + Sync {
    async fn find(&self, key: String) -> Result<Option<configuration::Model>, Box<dyn Error>>;
    async fn upsert(
        &self,
        key: String,
        value: String,
    ) -> Result<configuration::Model, Box<dyn Error>>;
}

pub struct ConfigurationServiceImpl {
    repository: Arc<dyn ConfigurationRepository>,
}

impl ConfigurationServiceImpl {
    pub fn new(repository: Arc<dyn ConfigurationRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl ConfigurationService for ConfigurationServiceImpl {
    async fn find(&self, key: String) -> Result<Option<configuration::Model>, Box<dyn Error>> {
        let r = self.repository.find_by_key(key.as_str()).await?;

        Ok(r)
    }

    async fn upsert(
        &self,
        key: String,
        value: String,
    ) -> Result<configuration::Model, Box<dyn Error>> {
        let r = self
            .repository
            .upsert(NewConfigurationModel { key, value })
            .await?;

        Ok(r)
    }
}
