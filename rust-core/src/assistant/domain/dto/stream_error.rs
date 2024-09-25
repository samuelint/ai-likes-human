use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StreamErrorDto {
    pub code: Option<String>,
    pub message: String,
    pub param: Option<String>,
    pub r#type: String,
}
