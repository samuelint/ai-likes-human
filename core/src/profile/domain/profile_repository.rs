use mockall::automock;
use std::error::Error;

use super::dto::ProfileDto;

#[automock]
#[async_trait::async_trait]
pub trait ProfileRepository: Sync + Send {
    async fn find(&self, id: &str) -> Result<Option<ProfileDto>, Box<dyn Error + Send>>;
    async fn find_by_name(&self, name: &str) -> Result<Option<ProfileDto>, Box<dyn Error + Send>>;
    async fn upsert(&self, model: &ProfileDto) -> Result<ProfileDto, Box<dyn Error + Send>>;
    async fn delete(&self, id: &str) -> Result<(), Box<dyn Error + Send>>;
}
