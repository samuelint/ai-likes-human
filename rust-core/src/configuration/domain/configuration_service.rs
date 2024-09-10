use std::error::Error;

use shaku::Provider;

use super::{dto::ConfigurationItemDto, repository::ConfigurationRepository};

pub trait Service {
    fn get_all(&mut self) -> Result<Vec<ConfigurationItemDto>, Box<dyn Error>>;
}

#[derive(Provider)]
#[shaku(interface = Service)]
pub struct ServiceImpl {
    #[shaku(provide)]
    repo: Box<dyn ConfigurationRepository>,
}

impl Service for ServiceImpl {
    fn get_all(&mut self) -> Result<Vec<ConfigurationItemDto>, Box<dyn Error>> {
        let mut result = self.repo.get_all()?;

        result.push(ConfigurationItemDto {
            id: 666,
            key: "mocked".to_string(),
            value: "from service".to_string(),
        });

        Ok(result)
    }
}
