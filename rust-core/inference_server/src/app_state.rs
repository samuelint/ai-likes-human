use std::sync::Arc;

pub use crate::api::{InvokeFn, StreamFn};

#[derive(Clone)]
pub struct ServerState {
    pub invoke_fn: Arc<InvokeFn>,
    pub stream_fn: Arc<StreamFn>,
}
