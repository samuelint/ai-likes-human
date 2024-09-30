use serde::{Deserialize, Serialize};

use super::annotation::MessageAnnotation;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DbMessageContent {
    Text { text: DbTextContent },
    ImageUrl { image_url: ImageUrlContent },
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

#[derive(Default, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ImageUrl {
    pub url: String,
    pub details: Option<String>, // auto, low, high
}

impl ImageUrl {
    pub fn url(url: &str) -> Self {
        Self {
            url: url.to_string(),
            details: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ImageUrlContent {
    pub image_url: ImageUrl,
}

impl ImageUrlContent {
    pub fn url(url: &str) -> Self {
        ImageUrlContent {
            image_url: ImageUrl {
                url: url.to_string(),
                ..ImageUrl::default()
            },
        }
    }
}

impl Default for ImageUrlContent {
    fn default() -> Self {
        Self {
            image_url: ImageUrl::default(),
        }
    }
}
