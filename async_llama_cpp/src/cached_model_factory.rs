use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use llama_cpp_2::{
    llama_backend::LlamaBackend,
    model::{params::LlamaModelParams, LlamaModel},
};

use anyhow::Result;

use super::{model_factory::ModelFactory, options::ModelOptions};

pub struct CachedModelFactory {
    backend: Arc<LlamaBackend>,
    model_cache: Mutex<HashMap<String, Arc<LlamaModel>>>,
}

impl CachedModelFactory {
    pub fn new(backend: Arc<LlamaBackend>) -> Self {
        Self {
            backend,
            model_cache: Mutex::new(HashMap::new()),
        }
    }

    pub fn load_model(
        &self,
        model_alias: &str,
        model_path: &str,
        options: &ModelOptions,
    ) -> Result<Arc<LlamaModel>> {
        let model_params = LlamaModelParams::default().with_n_gpu_layers(options.n_gpu_layers);
        let model = LlamaModel::load_from_file(&self.backend, model_path, &model_params)?;

        let model = Arc::new(model);
        self.model_cache
            .lock()
            .unwrap()
            .insert(model_alias.to_string(), model.clone());

        Ok(model)
    }

    pub fn load_model_path(
        &self,
        model_path: &str,
        options: &ModelOptions,
    ) -> Result<Arc<LlamaModel>> {
        self.load_model(model_path, model_path, options)
    }

    fn find_existing_model(&self, model_path: &str) -> Option<Arc<LlamaModel>> {
        let binding = self.model_cache.lock().unwrap();
        binding.get(model_path).map(Arc::clone)
    }

    fn get(&self, alias: &str) -> Result<Arc<LlamaModel>> {
        let model = match self.find_existing_model(alias) {
            Some(model) => model.clone(),
            None => return Err(anyhow::anyhow!("Model not found: {}", alias)),
        };

        Ok(model)
    }
}

impl ModelFactory for CachedModelFactory {
    fn create(&self, alias: &str) -> Result<Arc<LlamaModel>> {
        self.get(alias)
    }
}
