#[cfg(test)]
#[path = "./api_message_test.rs"]
mod api_message_test;

use crate::chat_completion::ChatCompletionMessageDto;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::{annotation::MessageAnnotation, DbCreateThreadMessageDto, Metadata};

#[derive(Default, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TextContentDto {
    pub value: String,
    pub annotations: Vec<MessageAnnotation>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TextMessageContentDto {
    pub r#type: String,
    pub text: TextContentDto,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ThreadMessageDto {
    pub id: String,
    pub object: String,
    pub created_at: i64,
    pub thread_id: Option<String>,
    pub status: String,
    pub role: String,
    pub content: Vec<MessageContent>,
    pub assistant_id: Option<String>,
    pub run_id: Option<String>,
    pub metadata: Option<Metadata>,
}

impl ThreadMessageDto {
    pub fn to_string_content(&self) -> String {
        self.content
            .iter()
            .filter_map(|c| match c {
                MessageContent::TextContentBlock(block) => Some(block),
                _ => None,
            })
            .map(|block| block.text.value.clone())
            .join("")
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TextContentBlock {
    pub text: TextDto,
}

impl TextContentBlock {
    pub fn new(text: &str) -> Self {
        Self {
            text: TextDto {
                value: text.to_string(),
                ..TextDto::default()
            },
            ..TextContentBlock::default()
        }
    }
}

impl Default for TextContentBlock {
    fn default() -> Self {
        Self {
            text: TextDto::default(),
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ImageURL {
    pub url: String,
    pub details: Option<String>, // auto, low, high
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ImageURLContentBlock {
    pub image_url: ImageURL,
}

impl Default for ImageURLContentBlock {
    fn default() -> Self {
        Self {
            image_url: ImageURL::default(),
            // r#type: "image_url".to_string(),
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TextDto {
    pub value: String,
    pub annotations: Vec<MessageAnnotation>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum MessageContent {
    #[serde(rename = "text")]
    TextContentBlock(TextContentBlock),
    #[serde(rename = "image_url")]
    ImageUrlContentBlock(ImageURLContentBlock),
}

impl MessageContent {
    pub fn new_text_content(value: &str) -> Self {
        MessageContent::TextContentBlock(TextContentBlock {
            text: TextDto {
                value: value.to_string(),
                ..TextDto::default()
            },
            ..TextContentBlock::default()
        })
    }

    pub fn new_image_url(url: &str) -> Self {
        MessageContent::ImageUrlContentBlock(ImageURLContentBlock {
            image_url: ImageURL {
                url: url.to_string(),
                ..ImageURL::default()
            },
            ..ImageURLContentBlock::default()
        })
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateThreadMessageDto {
    pub id: String,
    pub status: Option<String>,
    pub content: Option<Vec<MessageContent>>,
    pub assistant_id: Option<Option<String>>,
    pub metadata: Option<Option<Metadata>>,
}

impl From<ThreadMessageDto> for ChatCompletionMessageDto {
    fn from(dto: ThreadMessageDto) -> Self {
        ChatCompletionMessageDto {
            content: dto.to_string_content(),
            role: dto.role,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiCreateThreadMessageDto {
    pub content: Vec<MessageContent>,
    pub role: String,
    pub attachments: Option<String>,
    pub metadata: Option<Metadata>,
}

impl ApiCreateThreadMessageDto {
    pub fn user() -> Self {
        Self {
            role: "user".to_string(),
            ..Self::default()
        }
    }
}

impl From<&ApiCreateThreadMessageDto> for DbCreateThreadMessageDto {
    fn from(dto: &ApiCreateThreadMessageDto) -> Self {
        DbCreateThreadMessageDto {
            content: dto.content.clone(),
            role: dto.role.clone(),
            attachments: dto.attachments.clone(),
            metadata: dto.metadata.clone(),
            ..DbCreateThreadMessageDto::default()
        }
    }
}

impl Default for ApiCreateThreadMessageDto {
    fn default() -> Self {
        Self {
            content: vec![],
            role: "user".to_string(),
            attachments: None,
            metadata: None,
        }
    }
}
