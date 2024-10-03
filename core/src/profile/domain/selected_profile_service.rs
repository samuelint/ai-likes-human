use std::{error::Error, sync::Arc, vec};

use migration::SOFTWARE_ENGINEER_PROFILE_NAME;

use super::{dto::ProfileDto, ProfileRepository};

pub struct SelectedProfileService {
    profile_repository: Arc<dyn ProfileRepository>,
}

impl SelectedProfileService {
    pub fn new(profile_repository: Arc<dyn ProfileRepository>) -> Self {
        Self { profile_repository }
    }

    pub async fn find_selected_profiles(&self) -> Result<Vec<ProfileDto>, Box<dyn Error + Send>> {
        let profile = self
            .profile_repository
            .find_by_name(SOFTWARE_ENGINEER_PROFILE_NAME)
            .await?
            .unwrap();

        Ok(vec![profile])
    }
}
