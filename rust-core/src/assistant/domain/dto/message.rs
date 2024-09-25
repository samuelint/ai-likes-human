#[cfg(test)]
#[path = "./message_test.rs"]
mod message_test;

use crate::chat_completion::ChatCompletionMessageDto;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::{annotation::MessageAnnotation, Metadata};

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
    pub created_at: String,
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
                MessageContent::Text(block) => Some(block),
                _ => None,
            })
            .map(|block| block.text.value.clone())
            .join("")
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TextContentBlock {
    pub text: TextDto,
    pub r#type: String, // text
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
            r#type: "text".to_string(),
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
    pub r#type: String, // image_url
}

impl Default for ImageURLContentBlock {
    fn default() -> Self {
        Self {
            image_url: ImageURL::default(),
            r#type: "image_url".to_string(),
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TextDto {
    pub value: String,
    pub annotations: Vec<MessageAnnotation>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum MessageContent {
    Text(TextContentBlock),
    ImageUrl(ImageURLContentBlock),
}

impl MessageContent {
    pub fn new_text_content(value: &str) -> Self {
        MessageContent::Text(TextContentBlock {
            text: TextDto {
                value: value.to_string(),
                ..TextDto::default()
            },
            ..TextContentBlock::default()
        })
    }

    pub fn new_image_url(url: &str) -> Self {
        MessageContent::ImageUrl(ImageURLContentBlock {
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
pub struct CreateThreadMessageDto {
    pub content: Vec<MessageContent>,
    pub role: String,
    pub status: String,
    pub thread_id: Option<String>,
    pub run_id: Option<String>,
    pub attachments: Option<String>,
    pub metadata: Option<Metadata>,
}

impl CreateThreadMessageDto {
    pub fn user() -> Self {
        Self {
            role: "user".to_string(),
            ..Self::default()
        }
    }
}

impl Default for CreateThreadMessageDto {
    fn default() -> Self {
        Self {
            content: vec![],
            role: "user".to_string(),
            thread_id: None,
            run_id: None,
            attachments: None,
            status: "in_progress".to_string(),
            metadata: None,
        }
    }
}
