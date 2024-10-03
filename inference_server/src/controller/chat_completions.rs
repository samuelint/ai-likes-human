use app_core::chat_completion::{ChatCompletionMessageDto, ChatCompletionObject};
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

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ApiChatCompletionRequestDto {
    pub model: String,
    pub stream: Option<bool>,
    pub messages: Vec<ChatCompletionMessageDto>,
    pub max_tokens: Option<i32>,
    pub temperature: Option<f32>,
}

pub async fn run_chat_completions(
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
    extract::Json(payload): extract::Json<ApiChatCompletionRequestDto>,
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
    payload: ApiChatCompletionRequestDto,
) -> Result<Json<ChatCompletionObject>, Box<dyn Error + Send>> {
    let result = state
        .api
        .chat_completion_invoke(&payload.model, &payload.messages)
        .await?;

    Ok(Json(result))
}

fn run_stream_chat_completions(
    state: Arc<ServerState>,
    payload: ApiChatCompletionRequestDto,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let model = payload.model.clone();

    Sse::new(try_stream! {
        let mut stream = state.api.chat_completion_stream(&model, &payload.messages).await;
        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(chunk) => {
                    let json_chunk = serde_json::to_string(&chunk).unwrap();
                    yield Event::default().data(json_chunk);
                },
                Err(e) => yield Event::default().data(format!("Error: {}", e)),
            }
        }

        yield Event::default().event("done").data("[DONE]");
    })
    .keep_alive(KeepAlive::default())
}
