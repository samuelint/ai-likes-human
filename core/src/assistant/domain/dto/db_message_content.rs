use serde::{Deserialize, Serialize};

use crate::chat_completion::{ImageUrl, MessageAnnotation};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DbMessageContent {
    Text { text: DbTextContent },
    ImageUrl { image_url: ImageUrl },
}

impl DbMessageContent {
    pub fn text_annotated(value: &str) -> Self {
        Self::Text {
            text: DbTextContent {
                value: value.to_string(),
                ..DbTextContent::default()
            },
        }
    }
}

impl Default for DbMessageContent {
    fn default() -> Self {
        Self::Text {
            text: DbTextContent::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DbTextContent {
    pub value: String,
    pub annotations: Vec<MessageAnnotation>,
}

impl DbTextContent {
    pub fn annotated(value: &str) -> Self {
        Self {
            value: value.to_string(),
            annotations: vec![],
        }
    }
}

impl Default for DbTextContent {
    fn default() -> Self {
        Self {
            value: "".to_string(),
            annotations: vec![],
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AnnotatedTextDto {
    pub value: String,
    pub annotations: Vec<MessageAnnotation>,
}
