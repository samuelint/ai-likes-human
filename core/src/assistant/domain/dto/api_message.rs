#[cfg(test)]
#[path = "./api_message_test.rs"]
mod api_message_test;

use crate::chat_completion::ChatCompletionMessageDto;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::{
    annotation::MessageAnnotation, DbCreateThreadMessageDto, DbMessageContent, ImageUrlContent,
    Metadata,
};

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
    pub created_at: i64,
    pub thread_id: Option<String>,
    pub status: String,
    pub role: String,
    pub content: Vec<ApiMessageContent>,
    pub assistant_id: Option<String>,
    pub run_id: Option<String>,
    pub metadata: Option<Metadata>,
}

impl ThreadMessageDto {
    pub fn to_string_content(&self) -> String {
        self.content
            .iter()
            .filter_map(|c| match c {
                ApiMessageContent::Text { text } => Some(text.to_string()),
                _ => None,
            })
            .join("")
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ApiUpdateThreadMessageDto {
    pub id: String,
    pub content: Option<Vec<ApiMessageContent>>,
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
    pub content: Vec<ApiMessageContent>,
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
            content: dto.content.iter().map(DbMessageContent::from).collect(),
            role: dto.role.clone(),
            attachments: dto.attachments.clone(),
            metadata: dto.metadata.clone(),
            status: "completed".to_string(),
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

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ApiMessageContent {
    Text { text: TextContent },
    ImageUrl { image_url: ImageUrlContent },
}

impl Default for ApiMessageContent {
    fn default() -> Self {
        Self::Text {
            text: TextContent::Annotated {
                value: "".to_string(),
                annotations: vec![],
            },
        }
    }
}

impl ApiMessageContent {
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

impl From<&ApiMessageContent> for DbMessageContent {
    fn from(content: &ApiMessageContent) -> Self {
        match content {
            ApiMessageContent::Text { text } => {
                let text_content = text.to_string();
                DbMessageContent::text_annotated(&text_content)
            }
            ApiMessageContent::ImageUrl { image_url } => DbMessageContent::ImageUrl {
                image_url: image_url.clone(),
            },
        }
    }
}
