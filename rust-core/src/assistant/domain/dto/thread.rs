use serde::{Deserialize, Serialize};

use super::{message::CreateThreadMessageDto, Metadata};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct CreateThreadDto {
    pub metadata: Option<Metadata>,
    pub messages: Vec<CreateThreadMessageDto>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ThreadDto {
    pub id: String,
    pub created_at: i64,
    pub metadata: Metadata,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct UpdateThreadDto {
    pub metadata: Option<Metadata>,
}
