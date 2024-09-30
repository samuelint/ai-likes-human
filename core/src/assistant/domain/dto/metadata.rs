use serde_json::{Map, Value};

pub type Metadata = Map<String, Value>;

pub struct MetadataBuilder {}

impl MetadataBuilder {
    pub fn create_empty() -> Metadata {
        Map::new()
    }
}
