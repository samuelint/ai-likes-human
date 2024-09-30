use std::error::Error;

use super::dto::ConfigurationDto;

#[async_trait::async_trait]
pub trait ConfigurationRepository: Sync + Send {
    async fn get_all(&self) -> Result<Vec<ConfigurationDto>, Box<dyn Error>>;
    async fn find(&self, id: i32) -> Result<Option<ConfigurationDto>, Box<dyn Error>>;
    async fn find_by_key(&self, key: &str) -> Result<Option<ConfigurationDto>, Box<dyn Error>>;
    async fn upsert(&self, model: ConfigurationDto) -> Result<ConfigurationDto, Box<dyn Error>>;
}
