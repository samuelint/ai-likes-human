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
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, error::Error, sync::Arc};

use crate::app_state::ServerState;

use super::openai::{
    to_langchain_messages, OpenAIChatCompletionChunkObject, OpenAIChatCompletionObject,
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
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
    extract::Json(payload): extract::Json<ChatCompletionParameters>,
) -> impl IntoResponse {
    let use_stream = payload.stream.unwrap_or(false);

    if use_stream {
        match run_stream_chat_completions(state, payload).await {
            Ok(r) => return r.into_response(),
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    } else {
        match run_json_chat_completions(state, payload).await {
            Ok(r) => return r.into_response(),
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}

async fn run_json_chat_completions(
    state: Arc<ServerState>,
    payload: ChatCompletionParameters,
) -> Result<Json<OpenAIChatCompletionObject>, Box<dyn Error>> {
    let messages: Vec<langchain_rust::schemas::Message> = to_langchain_messages(payload.messages);
    let llm = state.api.get_llm_client(payload.model.clone())?;
    let result = llm.generate(&messages[..]).await?;

    let message = OpenAIMessage::new_assistant(result.generation);
    let data = OpenAIChatCompletionObject::new_single_choice(message, payload.model);

    Ok(Json(data))
}

async fn run_stream_chat_completions(
    state: Arc<ServerState>,
    payload: ChatCompletionParameters,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, Box<dyn Error>> {
    let model = payload.model.clone();
    let messages: Vec<langchain_rust::schemas::Message> = to_langchain_messages(payload.messages);
    let llm = state.api.get_llm_client(payload.model)?;
    let stream = llm.stream(&messages[..]).await?;

    let stream = stream.map(move |item| {
        let item = item.unwrap();
        let chunk =
            OpenAIChatCompletionChunkObject::new_assistant_chunk(item.content, model.clone());

        Ok(Event::default().data(format!("data: {}", serde_json::to_string(&chunk).unwrap())))
    });

    let last_chunk = once(async { Ok(Event::default().data("data: [DONE]")) });

    let stream = stream.chain(last_chunk);

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}
