use app_core::agent::domain::message_repository::NewMessageModel;
use axum::{extract, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::app_state::ServerState;

#[derive(Default, Serialize, Deserialize)]
pub struct CreateThreadDto {
    pub messages: Vec<NewMessageModel>,
    pub metadata: Option<serde_json::Value>,
}

pub async fn create_thread(
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
    extract::Json(payload): extract::Json<CreateThreadDto>,
) -> impl IntoResponse {
    todo!()
}

#[derive(Default, Serialize, Deserialize)]
pub struct ListThreadsDto {
    pub after: Option<i32>,
    pub before: Option<i32>,
    pub limit: Option<u64>,
}

pub async fn list_threads(
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
    extract::Json(payload): extract::Json<ListThreadsDto>,
) -> impl IntoResponse {
    todo!()
}

pub async fn find_thread(
    axum::extract::Path(thread_id): axum::extract::Path<i32>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
) -> impl IntoResponse {
    todo!()
}

#[derive(Default, Serialize, Deserialize)]
pub struct UpdateThreadDto {
    pub metadata: Option<serde_json::Value>,
}

pub async fn update_thread(
    axum::extract::Path(thread_id): axum::extract::Path<i32>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
    extract::Json(payload): extract::Json<UpdateThreadDto>,
) -> impl IntoResponse {
    todo!()
}

pub async fn delete_thread(
    axum::extract::Path(thread_id): axum::extract::Path<i32>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
) -> impl IntoResponse {
    todo!()
}

pub async fn list_thread_messages(
    axum::extract::Path(thread_id): axum::extract::Path<i32>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
) -> impl IntoResponse {
    todo!()
}

pub async fn find_thread_message(
    axum::extract::Path((thread_id, message_id)): axum::extract::Path<(i32, i32)>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
) -> impl IntoResponse {
    todo!()
}

pub async fn delete_thread_message(
    axum::extract::Path((thread_id, message_id)): axum::extract::Path<(i32, i32)>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
) -> impl IntoResponse {
    todo!()
}

#[derive(Default, Serialize, Deserialize)]
pub struct CreateThreadMessageDto {
    pub content: String,
    pub role: String,
    pub attachments: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

pub async fn create_thread_message(
    axum::extract::Path((thread_id, message_id)): axum::extract::Path<(i32, i32)>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
    extract::Json(payload): extract::Json<CreateThreadMessageDto>,
) -> impl IntoResponse {
    todo!()
}

#[derive(Default, Serialize, Deserialize)]
pub struct CreateThreadRunDto {
    pub assistant_id: String,
    pub model: String,
    pub temperature: Option<i32>,
    pub stream: Option<bool>,
}

pub async fn create_thread_run(
    axum::extract::Path(thread_id): axum::extract::Path<i32>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
    extract::Json(payload): extract::Json<CreateThreadRunDto>,
) -> impl IntoResponse {
    todo!()
}

pub async fn find_thread_run(
    axum::extract::Path((thread_id, run_id)): axum::extract::Path<(i32, i32)>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
) -> impl IntoResponse {
    todo!()
}

pub async fn list_thread_run(
    axum::extract::Path(i32): axum::extract::Path<i32>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
) -> impl IntoResponse {
    todo!()
}