use app_core::agent::domain::{
    dto::{
        CreateMessageDto, CreateRunDto, CreateThreadAndRunDto, CreateThreadDto, UpdateThreadDto,
    },
    CreateMessageParams, UpdateThreadParams,
};
pub use app_core::PageRequest;
use axum::{
    extract::{self, Query},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use std::sync::Arc;

use crate::app_state::ServerState;

pub async fn create_thread(
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
    extract::Json(payload): extract::Json<CreateThreadDto>,
) -> impl IntoResponse {
    let service = state.core_container.agent_module.get_thread_repository();

    match service.create(payload.into()).await {
        Ok(thread) => return Json(thread).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn create_thread_and_run(
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
    extract::Json(payload): extract::Json<CreateThreadAndRunDto>,
) -> impl IntoResponse {
    let service = state.core_container.agent_module.get_run_service();

    match service.create_thread_and_run(payload).await {
        Ok(thread) => return Json(thread).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn list_threads(
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
    Query(page_request): Query<PageRequest>,
) -> impl IntoResponse {
    let service = state.core_container.agent_module.get_thread_repository();

    match service.list_by_page(page_request).await {
        Ok(thread) => return Json(thread).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn find_thread(
    axum::extract::Path(thread_id): axum::extract::Path<i32>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
) -> impl IntoResponse {
    let service = state.core_container.agent_module.get_thread_repository();

    match service.find(thread_id).await {
        Ok(thread) => {
            if thread.is_none() {
                return (StatusCode::NOT_FOUND, "").into_response();
            }

            return Json(thread).into_response();
        }
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn update_thread(
    axum::extract::Path(thread_id): axum::extract::Path<i32>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
    extract::Json(payload): extract::Json<UpdateThreadDto>,
) -> impl IntoResponse {
    let service = state.core_container.agent_module.get_thread_repository();

    match service
        .update(UpdateThreadParams {
            id: thread_id,
            metadata: payload.metadata,
        })
        .await
    {
        Ok(thread) => return Json(thread).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn delete_thread(
    axum::extract::Path(thread_id): axum::extract::Path<i32>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
) -> impl IntoResponse {
    let service = state.core_container.agent_module.get_thread_repository();

    match service.delete(thread_id).await {
        Ok(_) => return ().into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn list_thread_messages(
    axum::extract::Path(thread_id): axum::extract::Path<i32>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
) -> impl IntoResponse {
    let service = state.core_container.agent_module.get_message_repository();

    match service.find_by_thread_id(thread_id).await {
        Ok(messages) => return Json(messages).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn find_thread_message(
    axum::extract::Path((_thread_id, message_id)): axum::extract::Path<(i32, i32)>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
) -> impl IntoResponse {
    let service = state.core_container.agent_module.get_message_repository();

    match service.find(message_id).await {
        Ok(message) => {
            if message.is_none() {
                return (StatusCode::NOT_FOUND, "").into_response();
            }

            return Json(message).into_response();
        }
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn delete_thread_message(
    axum::extract::Path((_thread_id, message_id)): axum::extract::Path<(i32, i32)>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
) -> impl IntoResponse {
    let service = state.core_container.agent_module.get_message_repository();

    match service.delete(message_id).await {
        Ok(_) => return ().into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn create_thread_message(
    axum::extract::Path(thread_id): axum::extract::Path<i32>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
    extract::Json(payload): extract::Json<CreateMessageDto>,
) -> impl IntoResponse {
    let service = state.core_container.agent_module.get_message_repository();

    match service
        .create(CreateMessageParams {
            thread_id: Some(thread_id),
            ..payload.into()
        })
        .await
    {
        Ok(message) => return Json(message).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn create_thread_run(
    axum::extract::Path(thread_id): axum::extract::Path<i32>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
    extract::Json(payload): extract::Json<CreateRunDto>,
) -> impl IntoResponse {
    let service = state.core_container.agent_module.get_run_service();

    match service.create(thread_id, payload).await {
        Ok(run) => return Json(run).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn find_thread_run(
    axum::extract::Path((_thread_id, run_id)): axum::extract::Path<(i32, i32)>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
) -> impl IntoResponse {
    let service = state.core_container.agent_module.get_run_repository();

    match service.find(run_id).await {
        Ok(run) => {
            if run.is_none() {
                return (StatusCode::NOT_FOUND, "").into_response();
            }

            return Json(run).into_response();
        }
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn list_thread_runs(
    axum::extract::Path(thread_id): axum::extract::Path<i32>,
    axum::extract::State(state): axum::extract::State<Arc<ServerState>>,
    Query(page_request): Query<PageRequest>,
) -> impl IntoResponse {
    let service = state.core_container.agent_module.get_run_repository();

    match service
        .list_by_thread_paginated(thread_id, page_request)
        .await
    {
        Ok(run) => return Json(run).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}