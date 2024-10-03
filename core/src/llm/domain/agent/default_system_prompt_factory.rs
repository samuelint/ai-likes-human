use std::{error::Error, sync::Arc};

use crate::profile::domain::{SelectedProfileService, SystemPromptBuilder};

pub struct DefaultSystemPromptFactory {
    selected_profile_service: Arc<SelectedProfileService>,
}

impl DefaultSystemPromptFactory {
    pub fn new(selected_profile_service: Arc<SelectedProfileService>) -> Self {
        Self {
            selected_profile_service,
        }
    }

    pub async fn create(&self) -> Result<String, Box<dyn Error + Send>> {
        let profiles = self
            .selected_profile_service
            .find_selected_profiles()
            .await?;

        let system_prompt = SystemPromptBuilder::new()
            .with_personal_assistant_role()
            .with_computer_info()
            .with_profiles(&profiles)
            .build();

        Ok(system_prompt)
    }
}
