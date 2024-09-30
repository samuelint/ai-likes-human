use std::error::Error;
use std::sync::Arc;

use crate::configuration::domain::dto::ConfigurationDto;
use crate::configuration::domain::ConfigurationRepository;

#[async_trait::async_trait]
pub trait ConfigurationService: Send + Sync {
    async fn find(&self, key: &str) -> Result<Option<ConfigurationDto>, Box<dyn Error + Send>>;
    async fn upsert(&self, key: &str, value: &str) -> Result<ConfigurationDto, Box<dyn Error + Send>>;
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
    async fn find(&self, key: &str) -> Result<Option<ConfigurationDto>, Box<dyn Error + Send>> {
        let r = self.repository.find_by_key(key).await?;

        Ok(r)
    }

    async fn upsert(&self, key: &str, value: &str) -> Result<ConfigurationDto, Box<dyn Error + Send>> {
        let r = self
            .repository
            .upsert(ConfigurationDto::new(key, value))
            .await?;

        Ok(r)
    }
}
