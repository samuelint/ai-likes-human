#[cfg(test)]
#[path = "./run_stream_test.rs"]
mod run_stream_test;
use std::error::Error;

use super::{
    message_delta::ThreadMessageDeltaDto, stream_error::StreamErrorDto, RunDto, RunStepDto,
    ThreadDto, ThreadMessageDto,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "object")]
pub enum ThreadEventData {
    #[serde(rename = "thread")]
    Thread(ThreadDto),

    #[serde(rename = "thread.run")]
    ThreadRun(RunDto),

    #[serde(rename = "thread.run.step")]
    ThreadRunStep(RunStepDto),

    #[serde(rename = "thread.message")]
    ThreadMessage(ThreadMessageDto),
    #[serde(rename = "thread.message.delta")]
    ThreadMessageDelta(ThreadMessageDeltaDto),

    #[serde(rename = "error")]
    Error(StreamErrorDto),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ThreadEventDto {
    pub data: ThreadEventData,
    pub event: String,
}

impl ThreadEventDto {
    pub fn thread_created(data: &ThreadDto) -> Self {
        Self {
            data: ThreadEventData::Thread(data.clone()),
            event: "thread.created".to_string(),
        }
    }
}

impl ThreadEventDto {
    pub fn thread_run_created(data: &RunDto) -> Self {
        Self {
            data: ThreadEventData::ThreadRun(data.clone()),
            event: "thread.run.created".to_string(),
        }
    }

    pub fn thread_run_queued(data: &RunDto) -> Self {
        Self {
            data: ThreadEventData::ThreadRun(data.clone()),
            event: "thread.run.queued".to_string(),
        }
    }

    pub fn thread_run_in_progress(data: &RunDto) -> Self {
        Self {
            data: ThreadEventData::ThreadRun(data.clone()),
            event: "thread.run.in_progress".to_string(),
        }
    }

    pub fn thread_run_completed(data: &RunDto) -> Self {
        Self {
            data: ThreadEventData::ThreadRun(data.clone()),
            event: "thread.run.completed".to_string(),
        }
    }
}

impl ThreadEventDto {
    pub fn run_step_created(data: &RunStepDto) -> Self {
        Self {
            data: ThreadEventData::ThreadRunStep(data.clone()),
            event: "thread.run.step.created".to_string(),
        }
    }

    pub fn run_step_in_progress(data: &RunStepDto) -> Self {
        Self {
            data: ThreadEventData::ThreadRunStep(data.clone()),
            event: "thread.run.step.in_progress".to_string(),
        }
    }

    pub fn run_step_completed(data: &RunStepDto) -> Self {
        Self {
            data: ThreadEventData::ThreadRunStep(data.clone()),
            event: "thread.run.step.completed".to_string(),
        }
    }
}

impl ThreadEventDto {
    pub fn thread_message_created(data: &ThreadMessageDto) -> Self {
        Self {
            data: ThreadEventData::ThreadMessage(data.clone()),
            event: "thread.message.created".to_string(),
        }
    }

    pub fn thread_message_in_progress(data: &ThreadMessageDto) -> Self {
        Self {
            data: ThreadEventData::ThreadMessage(data.clone()),
            event: "thread.message.in_progress".to_string(),
        }
    }

    pub fn thread_message_completed(data: &ThreadMessageDto) -> Self {
        Self {
            data: ThreadEventData::ThreadMessage(data.clone()),
            event: "thread.message.completed".to_string(),
        }
    }
}

impl ThreadEventDto {
    pub fn thread_message_delta(data: &ThreadMessageDeltaDto) -> Self {
        Self {
            data: ThreadEventData::ThreadMessageDelta(data.clone()),
            event: "thread.message.delta".to_string(),
        }
    }
}

impl ThreadEventDto {
    pub fn error(data: &StreamErrorDto) -> Self {
        Self {
            data: ThreadEventData::Error(data.clone()),
            event: "error".to_string(),
        }
    }

    pub fn std_error(error: Box<dyn Error>) -> Self {
        Self::error(&StreamErrorDto {
            message: error.to_string(),
            r#type: "error".to_string(),
            ..StreamErrorDto::default()
        })
    }
}
