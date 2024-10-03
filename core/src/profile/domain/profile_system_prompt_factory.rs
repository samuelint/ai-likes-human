use std::{error::Error, sync::Arc};

use super::{SelectedProfileService, SystemPromptBuilder};

pub struct ProfileSystemPromptFactory {
    selected_profile_service: Arc<SelectedProfileService>,
}

impl ProfileSystemPromptFactory {
    pub fn new(selected_profile_service: Arc<SelectedProfileService>) -> Self {
        Self {
            selected_profile_service,
        }
    }

    pub async fn create_system_prompt(&self) -> Result<String, Box<dyn Error + Send>> {
        let profiles = self
            .selected_profile_service
            .find_selected_profiles()
            .await?;

        let prompt = SystemPromptBuilder::new()
            .with_computer_info()
            .with_profiles(&profiles)
            .build();

        Ok(prompt)
    }
}
