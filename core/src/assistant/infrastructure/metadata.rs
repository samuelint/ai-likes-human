use serde_json::Map;

use crate::assistant::domain::dto::metadata::Metadata;

pub fn serialize_metadata_opt(metadata: Option<Metadata>) -> String {
    match metadata {
        Some(metadata) => serialize_metadata(&metadata),
        None => "".to_string(),
    }
}

pub fn serialize_metadata(metadata: &Metadata) -> String {
    match serde_json::to_string(metadata) {
        Ok(metadata) => metadata,
        Err(e) => {
            eprintln!("Error serializing metadata: {}", e);

            "".to_string()
        }
    }
}

pub fn deserialize_metadata(metadata: &str) -> Metadata {
    match serde_json::from_str(metadata) {
        Ok(metadata) => metadata,
        Err(e) => {
            eprintln!("Error deserializing metadata: {}", e);

            Map::new()
        }
    }
}
