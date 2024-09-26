use serde::{Deserialize, Serialize};

use super::{MessageContent, Metadata};

#[derive(Serialize, Deserialize, Clone)]
pub struct DbCreateThreadMessageDto {
    pub content: Vec<MessageContent>,
    pub role: String,
    pub status: String,
    pub thread_id: Option<String>,
    pub run_id: Option<String>,
    pub attachments: Option<String>,
    pub metadata: Option<Metadata>,
}

impl DbCreateThreadMessageDto {
    pub fn user() -> Self {
        Self {
            role: "user".to_string(),
            ..Self::default()
        }
    }
}

impl Default for DbCreateThreadMessageDto {
    fn default() -> Self {
        Self {
            content: vec![],
            role: "user".to_string(),
            thread_id: None,
            run_id: None,
            attachments: None,
            status: "in_progress".to_string(),
            metadata: None,
        }
    }
}
