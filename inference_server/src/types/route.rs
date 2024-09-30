use app_core::chat_completion::ChatCompletionMessageDto;
use async_stream::try_stream;
use futures::Stream;
use serde_json::Value;
use std::{error::Error, pin::Pin};

#[derive(Debug, Default, Clone)]
pub struct StreamData {
    pub value: Value,
    pub content: String,
}

impl StreamData {
    pub fn new<S: Into<String>>(value: Value, content: S) -> Self {
        Self {
            value,
            content: content.into(),
        }
    }
}

pub type InvokeResult = Result<String, Box<dyn Error>>;
pub type InvokeFn = dyn Fn(&[ChatCompletionMessageDto]) -> InvokeResult + Send + Sync;

pub fn default_invoke(_messages: &[ChatCompletionMessageDto]) -> InvokeResult {
    Ok("".to_string())
}

pub type StreamResult =
    Pin<Box<dyn Stream<Item = Result<StreamData, Box<dyn Error + Send>>> + Send>>;
pub type StreamFn = dyn Fn(&[ChatCompletionMessageDto]) -> StreamResult + Send + Sync;

pub fn default_stream(_messages: &[ChatCompletionMessageDto]) -> StreamResult {
    let stream = try_stream! {
        yield StreamData::new(
            serde_json::json!({}),
            "".to_string(),
            );
    };

    Box::pin(stream)
}
