use app_core::agent::domain::dto::CreateThreadAndRunDto;
use async_stream::try_stream;
use axum::response::{
    sse::{Event, KeepAlive},
    Sse,
};
use futures::{Stream, StreamExt};
use std::convert::Infallible;

use crate::app_state::ServerState;

pub fn stream_create_thread_and_run(
    state: &ServerState,
    dto: &CreateThreadAndRunDto,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let service = state.core_container.agent_module.get_stream_run_service();
    let dto = dto.clone();
    Sse::new(try_stream! {
        let mut stream = service.create_thread_run_and_stream(dto);
        while let Some(item) = stream.next().await {
            let data = item.unwrap();
            let json_data = serde_json::to_string(&data).unwrap();

            yield Event::default().data(json_data)
        }
    })
    .keep_alive(KeepAlive::default())
}
