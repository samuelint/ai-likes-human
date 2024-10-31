use std::{
    num::NonZeroU32,
    sync::{Arc, Mutex},
};

use llama_cpp_2::{
    context::{params::LlamaContextParams, LlamaContext},
    llama_backend::LlamaBackend,
    model::LlamaModel,
};

use anyhow::Result;

use super::options::ContextOptions;

struct ThreadSafeLlamaContext<'a> {
    pub inner: Arc<Mutex<LlamaContext<'a>>>,
}

pub trait ModelContextFactory: Send + Sync {
    fn create<'a>(&self, model: &'a LlamaModel) -> Result<ThreadSafeLlamaContext<'a>>;
}

pub struct ModelContextFactoryImpl {
    backend: Arc<LlamaBackend>,
    context_options: ContextOptions,
}

impl ModelContextFactoryImpl {
    pub fn new(backend: Arc<LlamaBackend>, context_options: ContextOptions) -> Self {
        Self {
            backend,
            context_options,
        }
    }

    pub fn create_context_from_model<'b>(&self, model: &'b LlamaModel) -> Result<LlamaContext<'b>> {
        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(Some(
                NonZeroU32::new(self.context_options.context_size).unwrap(),
            ))
            .with_seed(self.context_options.seed)
            .with_n_threads(self.context_options.n_threads)
            .with_n_threads_batch(self.context_options.n_threads_batch);

        let ctx = model.new_context(&self.backend, ctx_params)?;

        Ok(ctx)
    }
}

impl ModelContextFactory for ModelContextFactoryImpl {
    fn create<'a>(&self, model: &'a LlamaModel) -> Result<ThreadSafeLlamaContext<'a>> {
        let r = self.create_context_from_model(model)?;

        Ok(ThreadSafeLlamaContext {
            inner: Arc::new(Mutex::new(r)),
        })
    }
}
