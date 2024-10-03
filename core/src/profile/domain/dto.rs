use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileDto {
    pub name: String,
    pub prompt: String,
}
