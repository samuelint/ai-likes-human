use serde::{Deserialize, Serialize};

use crate::chat_completion::ApiMessageContent;

use super::{DbMessageContent, Metadata};

#[derive(Serialize, Deserialize, Clone)]
pub struct DbCreateThreadMessageDto {
    pub content: Vec<DbMessageContent>,
    pub role: String,
    pub status: String,
    pub thread_id: Option<String>,
    pub run_id: Option<String>,
    pub attachments: Option<String>,
    pub metadata: Option<Metadata>,
    pub assistant_id: Option<String>,
}

impl DbCreateThreadMessageDto {
    pub fn user() -> Self {
        Self {
            role: "user".to_string(),
            status: "completed".to_string(),
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
            assistant_id: None,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DbUpdateThreadMessageDto {
    pub status: Option<String>,
    pub content: Option<Vec<ApiMessageContent>>,
    pub assistant_id: Option<Option<String>>,
    pub metadata: Option<Option<Metadata>>,
}
