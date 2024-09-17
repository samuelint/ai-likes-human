use axum::{
    extract::{self},
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse,
    },
    Json,
};
use futures::{
    stream::{once, Stream},
    StreamExt,
};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, error::Error};

use crate::app_state::ServerState;

use super::{
    openai::{OpenAIChatCompletionChunkObject, OpenAIChatCompletionObject, OpenAIMessage},
    StreamData,
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
    axum::extract::State(state): axum::extract::State<ServerState>,
    extract::Json(payload): extract::Json<ChatCompletionParameters>,
) -> impl IntoResponse {
    let use_stream = payload.stream.unwrap_or(false);

    if use_stream {
        return run_stream_chat_completions(state, payload)
            .await
            .into_response();
    } else {
        return run_json_chat_completions(state, payload)
            .await
            .into_response();
    }
}

async fn run_json_chat_completions(
    state: ServerState,
    payload: ChatCompletionParameters,
) -> Json<OpenAIChatCompletionObject> {
    let messages = payload.messages;
    let invoke_fn = state.invoke_fn.as_ref();
    let result = invoke_fn(&messages[..]).unwrap();

    let message = OpenAIMessage::new_assistant(result.to_string());
    let data = OpenAIChatCompletionObject::new_single_choice(message);

    Json(data)
}

async fn run_stream_chat_completions(
    state: ServerState,
    payload: ChatCompletionParameters,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let messages = payload.messages;
    let create_invoke_stream = state.stream_fn.as_ref();
    let stream = create_invoke_stream(&messages[..]);

    let stream = stream.map(move |item: Result<StreamData, Box<dyn Error + Send>>| {
        let item = item.unwrap();
        let chunk = OpenAIChatCompletionChunkObject::new_assistant_chunk(
            item.content,
            payload.model.clone(),
        );

        Ok(Event::default().data(format!("data: {}", serde_json::to_string(&chunk).unwrap())))
    });

    let last_chunk = once(async { Ok(Event::default().data("data: [DONE]")) });

    let stream = stream.chain(last_chunk);

    Sse::new(stream).keep_alive(KeepAlive::default())
}
