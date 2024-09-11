use std::error::Error;

use shaku::Provider;

use crate::configuration::domain::{
    dto::{ConfigurationItemDto, NewConfigurationItemDto},
    repository::ConfigurationRepository,
};

pub trait ConfigurationService {
    fn find(&mut self, key: String) -> Result<Option<ConfigurationItemDto>, Box<dyn Error>>;
    fn upsert(
        &mut self,
        key: String,
        value: String,
    ) -> Result<ConfigurationItemDto, Box<dyn Error>>;
}

#[derive(Provider)]
#[shaku(interface = ConfigurationService)]
pub struct ConfigurationServiceImpl {
    #[shaku(provide)]
    repository: Box<dyn ConfigurationRepository>,
}

impl ConfigurationService for ConfigurationServiceImpl {
    fn find(&mut self, key: String) -> Result<Option<ConfigurationItemDto>, Box<dyn Error>> {
        self.repository.find_by_key(key.as_str())
    }

    fn upsert(
        &mut self,
        key: String,
        value: String,
    ) -> Result<ConfigurationItemDto, Box<dyn Error>> {
        self.repository
            .upsert_value_for_key(NewConfigurationItemDto {
                key: &key,
                value: &value,
            })
    }
}
