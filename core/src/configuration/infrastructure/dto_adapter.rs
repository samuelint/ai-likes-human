use crate::{configuration::domain::dto::ConfigurationDto, entities::configuration};

impl From<&configuration::Model> for ConfigurationDto {
    fn from(item: &configuration::Model) -> Self {
        ConfigurationDto {
            key: item.key.clone(),
            value: item.value.clone(),
        }
    }
}
