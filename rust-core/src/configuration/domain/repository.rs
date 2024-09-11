use std::error::Error;

use super::dto::{ConfigurationItemDto, NewConfigurationItemDto};

pub trait ConfigurationRepository {
    fn get_all(&mut self) -> Result<Vec<ConfigurationItemDto>, Box<dyn Error>>;
    fn find(&mut self, id: &i32) -> Result<Option<ConfigurationItemDto>, Box<dyn Error>>;
    fn find_by_key(&mut self, key: &str) -> Result<Option<ConfigurationItemDto>, Box<dyn Error>>;

    fn upsert_value_for_key(
        &mut self,
        item: NewConfigurationItemDto,
    ) -> Result<ConfigurationItemDto, Box<dyn Error>>;

    fn create(
        &mut self,
        item: NewConfigurationItemDto,
    ) -> Result<ConfigurationItemDto, Box<dyn Error>>;

    fn delete(&mut self, key: &str) -> Result<(), Box<dyn Error>>;
}
