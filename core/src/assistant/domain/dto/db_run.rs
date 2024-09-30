use serde::{Deserialize, Serialize};

use super::Metadata;

#[derive(Default, Serialize, Deserialize)]
pub struct DbCreateRunDto {
    pub assistant_id: String,
    pub thread_id: String,
    pub model: String,
    pub status: String,
    pub instructions: Option<String>,
    pub temperature: Option<i32>,
    pub metadata: Option<Metadata>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct DbUpdateRunDto {
    pub metadata: Option<Metadata>,
    pub status: Option<String>,
}
