use app_core::assistant::domain::dto::{ApiCreateRunDto, ApiCreateThreadAndRunDto};
use async_stream::try_stream;
use axum::response::{
    sse::{Event, KeepAlive},
    Sse,
};
use futures::{Stream, StreamExt};
use std::convert::Infallible;

use crate::app_state::ServerState;

use super::thread_event_adapter::{create_done_event, result_to_sse_event};

pub fn stream_create_thread_and_run(
    state: &ServerState,
    dto: &ApiCreateThreadAndRunDto,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let service = state.core_container.agent_module.get_stream_run_service();
    let dto = dto.clone();

    Sse::new(try_stream! {
        let mut stream = service.stream_new_thread(&dto);
        while let Some(item) = stream.next().await {
            yield result_to_sse_event(&item)
        }

        yield create_done_event();
    })
    .keep_alive(KeepAlive::default())
}

pub fn stream_create_thread_run(
    state: &ServerState,
    thread_id: &str,
    dto: &ApiCreateRunDto,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let service = state.core_container.agent_module.get_stream_run_service();
    let dto = dto.clone();
    let thread_id = thread_id.to_string();

    Sse::new(try_stream! {
        let mut stream = service.stream_new_run(&thread_id, &dto);
        while let Some(item) = stream.next().await {
            yield result_to_sse_event(&item)
        }

        yield create_done_event();
    })
    .keep_alive(KeepAlive::default())
}
