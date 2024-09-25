use super::dto::ThreadEvent;
use futures::Stream;
use std::{convert::Infallible, pin::Pin};

pub type AssistantStream = Pin<Box<dyn Stream<Item = Result<ThreadEvent, Infallible>> + Send>>;
