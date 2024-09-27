use std::convert::Infallible;

use app_core::assistant::domain::dto::ThreadEventDto;
use axum::response::sse::Event;

pub fn result_to_sse_event(event: &Result<ThreadEventDto, Infallible>) -> Event {
    match event {
        Ok(event) => to_sse_event(event),
        Err(e) => Event::default()
            .event("error")
            .data(format!("Error: {}", e)),
    }
}

pub fn to_sse_event(event: &ThreadEventDto) -> Event {
    match serde_json::to_string(&event.data) {
        Ok(serialized) => Event::default().event(event.event.clone()).data(serialized),
        Err(e) => {
            return Event::default()
                .event("error")
                .data(format!("Error: {}", e))
        }
    }
}
