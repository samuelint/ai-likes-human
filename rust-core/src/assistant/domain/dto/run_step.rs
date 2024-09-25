use serde::{Deserialize, Serialize};

use crate::utils::time::current_unix_time;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct RunStepDto<'a, TStepDetails> {
    pub id: i32,
    pub created_at: i64,
    pub assistant_id: String,
    pub thread_id: Option<i32>,
    pub run_id: Option<i32>,
    pub status: String,
    pub instructions: Option<String>,
    pub model: String,
    pub r#type: &'a str,
    pub step_details: TStepDetails,
}

impl<'a> RunStepDto<'a, MessageCreationStepDetails<'a>> {
    pub fn message_creation(
        id: i32,
        assistant_id: String,
        model: String,
        status: String,
        step_details: MessageCreationStepDetails<'a>,
        thread_id: Option<i32>,
        run_id: Option<i32>,
        instructions: Option<String>,
    ) -> Self {
        Self {
            id,
            created_at: current_unix_time(),
            assistant_id,
            thread_id,
            run_id,
            status,
            r#type: "message_creation",
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
pub struct MessageCreationStepDetails<'a> {
    pub message_creation: MessageCreation,
    pub r#type: &'a str, // message_creation
}

impl<'a> MessageCreationStepDetails<'a> {
    pub fn new(message_creation: MessageCreation) -> Self {
        Self {
            message_creation,
            r#type: "message_creation",
        }
    }
}

impl<'a> RunStepDto<'a, ToolCallsStepDetails<'a>> {
    pub fn tool_calls(
        id: i32,
        assistant_id: String,
        model: String,
        status: String,
        step_details: ToolCallsStepDetails<'a>,
        thread_id: Option<i32>,
        run_id: Option<i32>,
        instructions: Option<String>,
    ) -> Self {
        Self {
            id,
            created_at: current_unix_time(),
            assistant_id,
            thread_id,
            run_id,
            status,
            r#type: "tool_calls",
            model,
            step_details,
            instructions,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Function<'a> {
    arguments: &'a str,
    name: &'a str,
    output: Option<&'a str>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct FunctionToolCall<'a> {
    pub id: i32,
    pub function: Function<'a>,
    pub r#type: &'a str, // function
}

impl<'a> FunctionToolCall<'a> {
    pub fn new(id: i32, function: Function<'a>) -> Self {
        Self {
            id,
            function,
            r#type: "function",
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ToolCallsStepDetails<'a> {
    pub tool_calls: Vec<FunctionToolCall<'a>>,
    pub r#type: &'a str, // tool_calls
}

impl<'a> ToolCallsStepDetails<'a> {
    pub fn new(tool_calls: Vec<FunctionToolCall<'a>>) -> Self {
        Self {
            tool_calls,
            r#type: "tool_calls",
        }
    }
}
