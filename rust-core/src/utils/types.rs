use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct PageRequest {
    pub after: Option<u64>,
    pub before: Option<u64>,
    pub limit: Option<u64>,
}
