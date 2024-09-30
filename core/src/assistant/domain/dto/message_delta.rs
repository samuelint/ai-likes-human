#[cfg(test)]
#[path = "./message_delta_test.rs"]
mod message_delta_test;

use serde::{Deserialize, Serialize};

use super::{ImageUrlContent, TextContent};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum MessageDeltaContent {
    Text {
        index: i32,
        #[serde(rename = "type")]
        type_: String,
        text: TextContent,
    },
    ImageUrl {
        index: i32,
        #[serde(rename = "type")]
        type_: String,
        image_url: ImageUrlContent,
    },
}

impl MessageDeltaContent {
    pub fn text(text: &str) -> Self {
        Self::Text {
            index: 0,
            type_: "text".to_string(),
            text: TextContent::annotated(text),
        }
    }

    pub fn image_url(url: &str) -> Self {
        Self::ImageUrl {
            index: 0,
            type_: "image_url".to_string(),
            image_url: ImageUrlContent::url(url),
        }
    }
}

impl Default for MessageDeltaContent {
    fn default() -> Self {
        Self::Text {
            index: 0,
            type_: "text".to_string(),
            text: TextContent::Annotated {
                value: "".to_string(),
                annotations: vec![],
            },
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct MessageDeltaDto {
    pub role: String,
    pub content: Vec<MessageDeltaContent>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ThreadMessageDeltaDto {
    pub id: String,
    pub delta: MessageDeltaDto,
}
