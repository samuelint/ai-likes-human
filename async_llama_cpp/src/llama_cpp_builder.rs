use anyhow::Result;
use llama_cpp_2::llama_backend::LlamaBackend;
use std::sync::Arc;

use crate::{
    async_llama_cpp::AsyncLLamaCPP,
    cached_model_factory::CachedModelFactory,
    model_context_factory::ModelContextFactoryImpl,
    options::{ContextOptions, ModelOptions},
};

pub struct Builder {
    backend: Arc<LlamaBackend>,
    model_factory: Arc<CachedModelFactory>,
}

impl Builder {
    pub fn create_with_backend(backend: Arc<LlamaBackend>) -> Self {
        let model_factory = Arc::new(CachedModelFactory::new(backend.clone()));

        Self {
            backend,
            model_factory,
        }
    }

    pub fn create() -> Result<Self> {
        let backend = Arc::new(LlamaBackend::init()?);

        Ok(Self::create_with_backend(backend))
    }

    pub fn with_model(&self, alias: &str, path: &str, options: ModelOptions) -> Result<&Self> {
        self.model_factory.load_model(alias, path, &options)?;

        Ok(self)
    }

    pub fn build_with_options(&self, context_options: ContextOptions) -> AsyncLLamaCPP {
        let context_factory = Arc::new(ModelContextFactoryImpl::new(
            self.backend.clone(),
            context_options,
        ));

        AsyncLLamaCPP::new(self.model_factory.clone(), context_factory)
    }

    pub fn build(&self) -> AsyncLLamaCPP {
        self.build_with_options(ContextOptions::default())
    }
}
