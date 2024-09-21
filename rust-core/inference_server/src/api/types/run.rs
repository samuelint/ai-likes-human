pub use app_core::agent::domain::run_service::CreateRunDto;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct RunDto {
    pub id: i32,
    pub created_at: String,
    pub assistant_id: String,
    pub thread_id: Option<i32>,
    pub instructions: Option<String>,
    pub model: String,
    pub status: String,
    pub metadata: Option<String>,
    pub temperature: Option<i32>,
}
