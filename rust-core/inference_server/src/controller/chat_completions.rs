use app_core::agent::domain::dto::chat_completion::{
    ChatCompletionChunkObject, ChatCompletionMessageDto, ChatCompletionObject,
};
use async_stream::try_stream;
use axum::{
    extract::{self},
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse,
    },
    Json,
};
use futures::{stream::Stream, StreamExt};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, error::Error, sync::Arc};

use crate::app_state::ServerState;

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatCompletionParameters {
    pub model: String,
    pub stream: Option<bool>,
    pub messages: Vec<ChatCompletionMessageDto>,
    pub max_tokens: Option<i32>,
    pub temperature: Option<f32>,
}

pub async fn run_chat_completions(
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
    extract::Json(payload): extract::Json<ChatCompletionParameters>,
) -> impl IntoResponse {
    let use_stream = payload.stream.unwrap_or(false);

    if use_stream {
        run_stream_chat_completions(state, payload).into_response()
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
) -> Result<Json<ChatCompletionObject>, Box<dyn Error>> {
    let messages: Vec<langchain_rust::schemas::Message> =
        payload.messages.iter().map(|m| m.clone().into()).collect();
    let llm = state
        .api
        .get_llm_client(&payload.model)
        .map_err(|e| e as Box<dyn Error>)?;

    let result = llm.generate(&messages[..]).await?;

    let message = ChatCompletionMessageDto::new_assistant(result.generation);
    let data = ChatCompletionObject::new_single_choice(message, payload.model);

    Ok(Json(data))
}

fn run_stream_chat_completions(
    state: Arc<ServerState>,
    payload: ChatCompletionParameters,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let model = payload.model.clone();
    let messages: Vec<langchain_rust::schemas::Message> =
        payload.messages.iter().map(|m| m.clone().into()).collect();

    Sse::new(try_stream! {
        let llm = match state.api.get_llm_client(&model) {
            Ok(llm) => llm,
            Err(e) => {
                yield Event::default().data(format!("Error: {}", e));
                return;
            }
        };
        let mut stream = match llm.stream(&messages[..]).await {
            Ok(stream) => stream,
            Err(e) => {
                yield Event::default().data(format!("Error: {}", e));
                return;
            }
        };

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.unwrap();
            let chunk = ChatCompletionChunkObject::new_assistant_chunk(chunk.content, &model);
            let json_chunk = serde_json::to_string(&chunk).unwrap();

            yield Event::default().data(json_chunk)
        }

        yield Event::default().data("[DONE]");


    })
    .keep_alive(KeepAlive::default())
}
