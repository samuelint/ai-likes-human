use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Completion {
    pub id: i32,
    pub some: String,
}

pub async fn run_chat_completions() -> impl axum::response::IntoResponse {
    let data = Completion {
        id: 0,
        some: "hello".to_owned(),
    };

    axum::Json(data)
}
