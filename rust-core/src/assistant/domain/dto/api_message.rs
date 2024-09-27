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
                MessageContent::Text { text } => Some(text.to_string()),
                _ => None,
            })
            .join("")
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(untagged)]
pub enum TextContent {
    String(String),
    Annotated {
        value: String,
        annotations: Vec<MessageAnnotation>,
    },
}

impl TextContent {
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
            TextContent::String(text) => text.to_string(),
            TextContent::Annotated { value, .. } => value.to_string(),
        }
    }
}

impl Default for TextContent {
    fn default() -> Self {
        Self::Annotated {
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

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageContent {
    Text { text: TextContent },
    ImageUrl { image_url: ImageUrlContent },
}

impl MessageContent {
    pub fn text(value: &str) -> Self {
        Self::Text {
            text: TextContent::annotated(value),
        }
    }

    pub fn image_url(url: &str) -> Self {
        Self::ImageUrl {
            image_url: ImageUrlContent::url(url),
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ApiUpdateThreadMessageDto {
    pub id: String,
    pub content: Option<Vec<MessageContent>>,
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
