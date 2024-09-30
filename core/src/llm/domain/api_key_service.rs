#[cfg(test)]
#[path = "./api_key_service_tests.rs"]
mod api_key_service_tests;

use std::sync::Arc;

use anyhow::anyhow;
use mockall::automock;

use crate::configuration::domain::ConfigurationRepository;

#[automock]
#[async_trait::async_trait]
pub trait ApiKeyService: Sync + Send {
    async fn get_api_key(&self, key: &str) -> Result<String, Box<dyn std::error::Error + Send>>;
}

pub struct ApiKeyServiceImpl {
    configuration_repository: Arc<dyn ConfigurationRepository>,
}

impl ApiKeyServiceImpl {
    pub fn new(configuration_repository: Arc<dyn ConfigurationRepository>) -> Self {
        Self {
            configuration_repository,
        }
    }

    fn get_from_env(key: &str) -> Result<String, Box<dyn std::error::Error + Send>> {
        match std::env::var(key) {
            Ok(val) => Ok(val),
            Err(_) => Err(anyhow!("Key {} not found", key).into()),
        }
    }
}

#[async_trait::async_trait]
impl ApiKeyService for ApiKeyServiceImpl {
    async fn get_api_key(&self, key: &str) -> Result<String, Box<dyn std::error::Error + Send>> {
        let config_value = self.configuration_repository.find_by_key(key).await?;

        match config_value {
            Some(config) => {
                if config.value.is_empty() {
                    Self::get_from_env(key)
                } else {
                    Ok(config.value)
                }
            }
            None => Self::get_from_env(key),
        }
    }
}
