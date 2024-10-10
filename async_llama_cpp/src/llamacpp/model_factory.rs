use std::sync::Arc;

use llama_cpp_2::model::LlamaModel;

use anyhow::Result;

pub trait ModelFactory: Send + Sync {
    fn create(&self, model_path: &str) -> Result<Arc<LlamaModel>>;
}
