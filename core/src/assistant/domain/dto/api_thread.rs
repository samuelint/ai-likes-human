use serde::{Deserialize, Serialize};

use super::{api_message::ApiCreateThreadMessageDto, Metadata};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ApiCreateThreadDto {
    pub metadata: Option<Metadata>,
    pub messages: Vec<ApiCreateThreadMessageDto>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ThreadDto {
    pub id: String,
    pub created_at: i64,
    pub metadata: Metadata,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ApiUpdateThreadDto {
    pub metadata: Option<Metadata>,
}
