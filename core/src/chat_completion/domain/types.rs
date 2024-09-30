use std::{error::Error, pin::Pin};

use futures::Stream;

use super::dto::{ChatCompletionChunkObject, ChatCompletionObject};

pub type ChatCompletionResult = Result<ChatCompletionObject, Box<dyn Error>>;
pub type ChatCompletionStream =
    Pin<Box<dyn Stream<Item = Result<ChatCompletionChunkObject, Box<dyn Error + Send>>> + Send>>;
