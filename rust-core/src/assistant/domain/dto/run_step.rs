use serde::{Deserialize, Serialize};

use crate::utils::time::TimeBuilder;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RunStep {
    ToolCallRunStep(RunStepDto<ToolCallsStepDetails>),
    MessageCreationRunStep(RunStepDto<MessageCreationStepDetails>),
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct RunStepDto<TStepDetails> {
    pub id: i32,
    pub created_at: i64,
    pub assistant_id: String,
    pub thread_id: Option<i32>,
    pub run_id: Option<i32>,
    pub status: String,
    pub instructions: Option<String>,
    pub model: String,
    pub r#type: String,
    pub step_details: TStepDetails,
}

impl RunStepDto<MessageCreationStepDetails> {
    pub fn message_creation(
        id: i32,
        assistant_id: String,
        model: String,
        status: String,
        step_details: MessageCreationStepDetails,
        thread_id: Option<i32>,
        run_id: Option<i32>,
        instructions: Option<String>,
    ) -> Self {
        Self {
            id,
            created_at: TimeBuilder::now().into(),
            assistant_id,
            thread_id,
            run_id,
            status,
            r#type: "message_creation".to_string(),
            model,
            step_details,
            instructions,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct MessageCreation {
    pub message_id: i32,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct MessageCreationStepDetails {
    pub message_creation: MessageCreation,
    pub r#type: String, // message_creation
}

impl MessageCreationStepDetails {
    pub fn new(message_creation: MessageCreation) -> Self {
        Self {
            message_creation,
            r#type: "message_creation".to_string(),
        }
    }
}

impl RunStepDto<ToolCallsStepDetails> {
    pub fn tool_calls(
        id: i32,
        assistant_id: String,
        model: String,
        status: String,
        step_details: ToolCallsStepDetails,
        thread_id: Option<i32>,
        run_id: Option<i32>,
        instructions: Option<String>,
    ) -> Self {
        Self {
            id,
            created_at: TimeBuilder::now().into(),
            assistant_id,
            thread_id,
            run_id,
            status,
            r#type: "tool_calls".to_string(),
            model,
            step_details,
            instructions,
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
