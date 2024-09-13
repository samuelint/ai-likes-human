use async_stream::try_stream;
use axum::{
    extract,
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse,
    },
    Json,
};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

use super::types::{
    OpenAIChatCompletionChoice, OpenAIChatCompletionObject, OpenAIChatCompletionUsage,
    OpenAIMessage,
};

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionParameters {
    pub model: String,
    pub stream: Option<bool>,
    pub messages: Vec<OpenAIMessage>,
    pub max_tokens: Option<i32>,
    pub temperature: Option<f32>,
}

pub async fn run_chat_completions(
    extract::Json(payload): extract::Json<ChatCompletionParameters>,
) -> impl IntoResponse {
    let use_stream = payload.stream.unwrap_or(false);

    if use_stream {
        return run_stream_chat_completions(payload).await.into_response();
    } else {
        return run_json_chat_completions(payload).await.into_response();
    }
}

async fn run_json_chat_completions(
    payload: ChatCompletionParameters,
) -> Json<OpenAIChatCompletionObject> {
    let choice1 = OpenAIChatCompletionChoice {
        message: Some(OpenAIMessage::new_assistant("salut".to_string())),
        ..OpenAIChatCompletionChoice::default()
    };

    let data = OpenAIChatCompletionObject {
        id: Some("1".to_string()),
        model: payload.model,
        choices: vec![choice1],
        ..OpenAIChatCompletionObject::default()
    };

    Json(data)
}

#[derive(Default, Serialize, Deserialize)]
pub struct OpenAIChatCompletionChunkChoice {
    pub index: i32,
    pub delta: Option<OpenAIMessage>,
    pub finish_reason: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct OpenAIChatCompletionChunkObject {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub system_fingerprint: Option<String>,
    pub choices: Vec<OpenAIChatCompletionChunkChoice>,
    pub usage: OpenAIChatCompletionUsage,
}

async fn run_stream_chat_completions(
    payload: ChatCompletionParameters,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let choice1 = OpenAIChatCompletionChunkChoice {
        delta: Some(OpenAIMessage::new_assistant("salut".to_string())),
        ..OpenAIChatCompletionChunkChoice::default()
    };
    let chunk = OpenAIChatCompletionChunkObject {
        id: "123".to_string(),
        model: payload.model.clone(),
        choices: vec![choice1],
        ..OpenAIChatCompletionChunkObject::default()
    };
    let final_choice = OpenAIChatCompletionChunkChoice {
        delta: None,
        ..OpenAIChatCompletionChunkChoice::default()
    };
    let final_chunk = OpenAIChatCompletionChunkObject {
        id: "123".to_string(),
        model: payload.model,
        choices: vec![final_choice],
        ..OpenAIChatCompletionChunkObject::default()
    };

    Sse::new(try_stream! {
        yield Event::default().data(format!("data: {}", serde_json::to_string(&chunk).unwrap()));
        yield Event::default().data(format!(
            "data: {}",
            serde_json::to_string(&final_chunk).unwrap()
        ));
        yield Event::default().data("data: [DONE]");
    })
    .keep_alive(KeepAlive::default())
}
