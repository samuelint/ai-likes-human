#[cfg(test)]
#[path = "./langchain_adapter_test.rs"]
mod langchain_adapter_test;

use langchain_rust::schemas::ImageContent;

use crate::llm::domain::message_type_adapter::to_langchain_message_type;

use super::{ChatCompletionMessageDto, ImageUrl};

impl From<ChatCompletionMessageDto> for langchain_rust::schemas::Message {
    fn from(message: ChatCompletionMessageDto) -> Self {
        let content = message.to_string_content();
        let image_urls = message.to_images_url_vec();
        let images: Vec<ImageContent> = image_urls
            .iter()
            .map(|image_url| image_url.into())
            .collect();

        langchain_rust::schemas::Message {
            content,
            message_type: to_langchain_message_type(message.role.clone()),
            id: None,
            tool_calls: None,
            images: Some(images),
        }
    }
}

impl From<&ImageUrl> for langchain_rust::schemas::ImageContent {
    fn from(image_url: &ImageUrl) -> Self {
        langchain_rust::schemas::ImageContent {
            image_url: image_url.url.clone(),
            detail: image_url.details.clone(),
        }
    }
}
