use serde::{Deserialize, Serialize};

use super::MessageAnnotation;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ApiMessageContent {
    Text { text: ApiTextContent },
    ImageUrl { image_url: ImageUrl },
}

impl Default for ApiMessageContent {
    fn default() -> Self {
        Self::Text {
            text: ApiTextContent::Annotated {
                value: "".to_string(),
                annotations: vec![],
            },
        }
    }
}

impl ApiMessageContent {
    pub fn text(value: &str) -> Self {
        Self::Text {
            text: ApiTextContent::annotated(value),
        }
    }

    pub fn image_url(url: &str) -> Self {
        Self::ImageUrl {
            image_url: ImageUrl::url(url),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(untagged)]
pub enum ApiTextContent {
    String(String),
    Annotated {
        value: String,
        annotations: Vec<MessageAnnotation>,
    },
}

impl ApiTextContent {
    pub fn annotated(text: &str) -> Self {
        Self::Annotated {
            value: text.to_string(),
            annotations: vec![],
        }
    }

    pub fn string(text: &str) -> Self {
        Self::String(text.to_string())
    }

    pub fn to_string(&self) -> String {
        match self {
            ApiTextContent::String(text) => text.to_string(),
            ApiTextContent::Annotated { value, .. } => value.to_string(),
        }
    }
}

impl Default for ApiTextContent {
    fn default() -> Self {
        Self::Annotated {
            value: "".to_string(),
            annotations: vec![],
        }
    }
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
