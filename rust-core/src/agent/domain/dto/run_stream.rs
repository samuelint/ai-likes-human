use std::error::Error;

use super::{stream_error::StreamErrorDto, RunDto, ThreadDto};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ThreadEvent {
    ThreadCreated(ThreadEventDto<ThreadDto>),
    ThreadRunCreated(ThreadEventDto<RunDto>),
    ThreadRunInProgress(ThreadEventDto<RunDto>),
    ThreadRunCompleted(ThreadEventDto<RunDto>),
    Error(ThreadEventDto<StreamErrorDto>),
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ThreadEventDto<TEvent> {
    pub data: TEvent,
    pub event: String,
}

impl ThreadEventDto<ThreadDto> {
    pub fn created_thread(data: ThreadDto) -> Self {
        Self {
            data,
            event: "thread.created".to_string(),
        }
    }
}

impl ThreadEventDto<RunDto> {
    pub fn created_run(data: RunDto) -> Self {
        Self {
            data,
            event: "thread.run.created".to_string(),
        }
    }

    pub fn in_progress(data: RunDto) -> Self {
        Self {
            data,
            event: "thread.run.in_progress".to_string(),
        }
    }

    pub fn completed(data: RunDto) -> Self {
        Self {
            data,
            event: "thread.run.completed".to_string(),
        }
    }
}

impl ThreadEventDto<StreamErrorDto> {
    pub fn error(data: StreamErrorDto) -> Self {
        Self {
            data,
            event: "error".to_string(),
        }
    }

    pub fn std_error(error: Box<dyn Error>) -> Self {
        Self::error(StreamErrorDto {
            message: error.to_string(),
            r#type: "error".to_string(),
            ..StreamErrorDto::default()
        })
    }
}
