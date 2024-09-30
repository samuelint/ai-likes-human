use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct PageRequest {
    pub after: Option<String>,
    pub before: Option<String>,
    pub limit: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct PageResponse<TData> {
    pub first_id: String,
    pub last_id: String,
    pub has_more: bool,
    pub data: Vec<TData>,
}
