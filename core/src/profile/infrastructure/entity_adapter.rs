use sea_orm::Set;

use crate::{entities::profile, profile::domain::dto::ProfileDto};

impl From<&profile::Model> for ProfileDto {
    fn from(model: &profile::Model) -> Self {
        Self {
            name: model.name.clone(),
            prompt: model.prompt.clone(),
        }
    }
}

impl From<&ProfileDto> for profile::ActiveModel {
    fn from(model: &ProfileDto) -> Self {
        Self {
            name: Set(model.name.clone()),
            prompt: Set(model.prompt.clone()),
            ..Default::default()
        }
    }
}
