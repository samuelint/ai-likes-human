use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct PageRequest {
    pub after: Option<u64>,
    pub before: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct PageResponse<TData> {
    pub first_id: String,
    pub last_id: String,
    pub has_more: bool,
    pub data: Vec<TData>,
}
