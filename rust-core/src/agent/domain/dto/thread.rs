use serde::{Deserialize, Serialize};

use super::message::CreateMessageDto;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct CreateThreadDto {
    pub metadata: Option<String>,
    pub messages: Vec<CreateMessageDto>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ThreadDto {
    pub id: String,
    pub created_at: String,
    pub metadata: String,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct UpdateThreadDto {
    pub metadata: Option<String>,
}
