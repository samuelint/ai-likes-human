use serde::{Deserialize, Serialize};

use super::{DbCreateThreadMessageDto, Metadata};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct DbCreateThreadDto {
    pub metadata: Option<Metadata>,
    pub messages: Vec<DbCreateThreadMessageDto>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct DbUpdateThreadDto {
    pub id: String,
    pub metadata: Option<Metadata>,
}
