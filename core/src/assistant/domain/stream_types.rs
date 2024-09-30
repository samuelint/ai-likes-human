use super::dto::ThreadEventDto;
use futures::Stream;
use std::{convert::Infallible, pin::Pin};

pub type AssistantStream = Pin<Box<dyn Stream<Item = Result<ThreadEventDto, Infallible>> + Send>>;
