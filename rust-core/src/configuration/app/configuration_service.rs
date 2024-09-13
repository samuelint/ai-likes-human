use std::error::Error;

use shaku::Provider;

use crate::configuration::domain::repository::{ConfigurationRepository, NewModel};
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

#[derive(Provider)]
#[shaku(interface = ConfigurationService)]
pub struct ConfigurationServiceImpl {
    #[shaku(provide)]
    repository: Box<dyn ConfigurationRepository>,
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
            .upsert_value_for_key(NewModel { key, value })
            .await?;

        Ok(r)
    }
}
