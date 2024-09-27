use serde::{Deserialize, Serialize};

use crate::utils::time::TimeBuilder;

use super::RunDto;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RunStepDetails {
    ToolCallRunStep(ToolCallsStepDetails),
    MessageCreationRunStep(MessageCreationStepDetails),
}

impl Default for RunStepDetails {
    fn default() -> Self {
        RunStepDetails::MessageCreationRunStep(MessageCreationStepDetails::default())
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct RunStepDto {
    pub id: String,
    pub created_at: i64,
    pub assistant_id: String,
    pub thread_id: Option<String>,
    pub run_id: Option<String>,
    pub status: String,
    pub instructions: Option<String>,
    pub model: String,
    pub r#type: String,
    pub step_details: RunStepDetails,
}

impl RunStepDto {
    pub fn message_creation_from_run(
        step_id: &str,
        message_id: &str,
        status: &str,
        run: &RunDto,
    ) -> Self {
        let thread_id = run.thread_id.as_deref();

        Self::message_creation(
            step_id,
            &run.assistant_id,
            &run.model,
            status,
            MessageCreationStepDetails::for_message_id(message_id),
            thread_id,
            Some(&run.id),
            None,
        )
    }

    pub fn message_creation(
        id: &str,
        assistant_id: &str,
        model: &str,
        status: &str,
        step_details: MessageCreationStepDetails,
        thread_id: Option<&str>,
        run_id: Option<&str>,
        instructions: Option<&str>,
    ) -> Self {
        Self {
            id: id.to_string(),
            created_at: TimeBuilder::now().into(),
            assistant_id: assistant_id.to_string(),
            thread_id: thread_id.map(|t| t.to_string()),
            run_id: run_id.map(|r| r.to_string()),
            status: status.to_string(),
            r#type: "message_creation".to_string(),
            model: model.to_string(),
            step_details: RunStepDetails::MessageCreationRunStep(step_details),
            instructions: instructions.map(|i| i.to_string()),
        }
    }

    pub fn tool_calls(
        id: &str,
        assistant_id: &str,
        model: &str,
        status: &str,
        step_details: ToolCallsStepDetails,
        thread_id: Option<&str>,
        run_id: Option<&str>,
        instructions: Option<&str>,
    ) -> Self {
        Self {
            id: id.to_string(),
            created_at: TimeBuilder::now().into(),
            assistant_id: assistant_id.to_string(),
            thread_id: thread_id.map(|t| t.to_string()),
            run_id: run_id.map(|r| r.to_string()),
            status: status.to_string(),
            r#type: "tool_calls".to_string(),
            model: model.to_string(),
            step_details: RunStepDetails::ToolCallRunStep(step_details),
            instructions: instructions.map(|i| i.to_string()),
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct MessageCreation {
    pub message_id: String,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct MessageCreationStepDetails {
    pub message_creation: MessageCreation,
    pub r#type: String, // message_creation
}

impl MessageCreationStepDetails {
    pub fn for_message_id(message_id: &str) -> Self {
        Self {
            message_creation: MessageCreation {
                message_id: message_id.to_string(),
            },
            r#type: "message_creation".to_string(),
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Function {
    arguments: String,
    name: String,
    output: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct FunctionToolCall {
    pub id: i32,
    pub function: Function,
    pub r#type: String, // function
}

impl FunctionToolCall {
    pub fn new(id: i32, function: Function) -> Self {
        Self {
            id,
            function,
            r#type: "function".to_string(),
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ToolCallsStepDetails {
    pub tool_calls: Vec<FunctionToolCall>,
    pub r#type: String, // tool_calls
}

impl ToolCallsStepDetails {
    pub fn new(tool_calls: Vec<FunctionToolCall>) -> Self {
        Self {
            tool_calls,
            r#type: "tool_calls".to_string(),
        }
    }
}
