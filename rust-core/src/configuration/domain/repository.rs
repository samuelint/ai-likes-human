use std::error::Error;

use crate::entities::configuration;

pub struct NewModel {
    pub key: String,
    pub value: String,
}

#[async_trait::async_trait]
pub trait ConfigurationRepository: Sync + Send {
    async fn get_all(&self) -> Result<Vec<configuration::Model>, Box<dyn Error>>;
    async fn find(&self, id: i32) -> Result<Option<configuration::Model>, Box<dyn Error>>;
    async fn find_by_key(&self, key: &str) -> Result<Option<configuration::Model>, Box<dyn Error>>;
    async fn upsert_value_for_key(
        &self,
        model: NewModel,
    ) -> Result<configuration::Model, Box<dyn Error>>;
}
