use std::error::Error;

use super::{
    message_delta::ThreadMessageDeltaDto, stream_error::StreamErrorDto, RunDto, RunStep, ThreadDto,
    ThreadMessageDto,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ThreadEvent {
    ThreadCreated(ThreadEventDto<ThreadDto>),

    ThreadRunCreated(ThreadEventDto<RunDto>),
    ThreadRunInProgress(ThreadEventDto<RunDto>),
    ThreadRunCompleted(ThreadEventDto<RunDto>),

    ThreadRunStepCreated(ThreadEventDto<RunStep>),
    ThreadRunStepInProgress(ThreadEventDto<RunStep>),
    ThreadRunStepCompleted(ThreadEventDto<RunStep>),

    ThreadMessageCreated(ThreadEventDto<ThreadMessageDto>),
    ThreadMessageInProgress(ThreadEventDto<ThreadMessageDto>),
    ThreadMessageDelta(ThreadEventDto<ThreadMessageDeltaDto>),
    ThreadMessageCompleted(ThreadEventDto<ThreadMessageDto>),

    Done(ThreadEventDto<String>),

    Error(ThreadEventDto<StreamErrorDto>),
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ThreadEventDto<TEvent> {
    pub data: TEvent,
    pub event: String,
}

impl ThreadEventDto<ThreadDto> {
    pub fn created_thread(data: &ThreadDto) -> Self {
        Self {
            data: data.clone(),
            event: "thread.created".to_string(),
        }
    }
}

impl ThreadEventDto<RunDto> {
    pub fn created_run(data: &RunDto) -> Self {
        Self {
            data: data.clone(),
            event: "thread.run.created".to_string(),
        }
    }

    pub fn in_progress(data: &RunDto) -> Self {
        Self {
            data: data.clone(),
            event: "thread.run.in_progress".to_string(),
        }
    }

    pub fn completed(data: &RunDto) -> Self {
        Self {
            data: data.clone(),
            event: "thread.run.completed".to_string(),
        }
    }
}

impl ThreadEventDto<RunStep> {
    pub fn run_step_created(data: &RunStep) -> Self {
        Self {
            data: data.clone(),
            event: "thread.run.step.created".to_string(),
        }
    }

    pub fn run_step_in_progress(data: &RunStep) -> Self {
        Self {
            data: data.clone(),
            event: "thread.run.step.in_progress".to_string(),
        }
    }

    pub fn run_step_completed(data: &RunStep) -> Self {
        Self {
            data: data.clone(),
            event: "thread.run.step.completed".to_string(),
        }
    }
}

impl ThreadEventDto<ThreadMessageDto> {
    pub fn thread_message_created(data: &ThreadMessageDto) -> Self {
        Self {
            data: data.clone(),
            event: "thread.message.created".to_string(),
        }
    }

    pub fn thread_message_in_progress(data: &ThreadMessageDto) -> Self {
        Self {
            data: data.clone(),
            event: "thread.message.in_progress".to_string(),
        }
    }

    pub fn thread_message_completed(data: &ThreadMessageDto) -> Self {
        Self {
            data: data.clone(),
            event: "thread.message.completed".to_string(),
        }
    }
}

impl ThreadEventDto<ThreadMessageDeltaDto> {
    pub fn thread_message_delta(data: &ThreadMessageDeltaDto) -> Self {
        Self {
            data: data.clone(),
            event: "thread.message.delta".to_string(),
        }
    }
}

impl ThreadEventDto<StreamErrorDto> {
    pub fn error(data: &StreamErrorDto) -> Self {
        Self {
            data: data.clone(),
            event: "error".to_string(),
        }
    }

    pub fn std_error(error: Box<dyn Error>) -> Self {
        Self {
            data: StreamErrorDto {
                message: error.to_string(),
                r#type: "error".to_string(),
                ..StreamErrorDto::default()
            },
            event: "error".to_string(),
        }
    }
}

impl ThreadEventDto<String> {
    pub fn done(data: &str) -> Self {
        Self {
            data: data.to_string(),
            event: "done".to_string(),
        }
    }
}
