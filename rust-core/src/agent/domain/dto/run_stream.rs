use super::{stream_error::StreamErrorDto, RunDto, ThreadDto};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug)]
pub enum ThreadEvent<'a> {
    ThreadCreated(ThreadEventDto<'a, ThreadDto>),
    ThreadRunCreated(ThreadEventDto<'a, RunDto>),
    ThreadRunInProgress(ThreadEventDto<'a, RunDto>),
    ThreadRunCompleted(ThreadEventDto<'a, RunDto>),
    Error(ThreadEventDto<'a, StreamErrorDto>),
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ThreadEventDto<'a, TEvent> {
    data: TEvent,
    event: &'a str,
}

impl<'a> ThreadEventDto<'a, ThreadDto> {
    pub fn created_thread(data: ThreadDto) -> Self {
        Self {
            data,
            event: "thread.created",
        }
    }
}

impl<'a> ThreadEventDto<'a, RunDto> {
    pub fn created_run(data: RunDto) -> Self {
        Self {
            data,
            event: "thread.run.created",
        }
    }

    pub fn in_progress(data: RunDto) -> Self {
        Self {
            data,
            event: "thread.run.in_progress",
        }
    }

    pub fn completed(data: RunDto) -> Self {
        Self {
            data,
            event: "thread.run.completed",
        }
    }
}

impl<'a> ThreadEventDto<'a, StreamErrorDto> {
    pub fn error(data: StreamErrorDto) -> Self {
        Self {
            data,
            event: "error",
        }
    }
}
