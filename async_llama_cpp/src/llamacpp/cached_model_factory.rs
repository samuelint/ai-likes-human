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
    new_model_options: ModelOptions,
    model_cache: Mutex<HashMap<String, Arc<LlamaModel>>>,
}

impl CachedModelFactory {
    pub fn new(backend: Arc<LlamaBackend>, new_model_options: ModelOptions) -> Self {
        Self {
            backend,
            new_model_options,
            model_cache: Mutex::new(HashMap::new()),
        }
    }

    fn create_new_model(&self, model_path: &str, options: &ModelOptions) -> Result<LlamaModel> {
        let model_params = LlamaModelParams::default().with_n_gpu_layers(options.n_gpu_layers);
        let model = LlamaModel::load_from_file(&self.backend, model_path, &model_params)?;

        Ok(model)
    }

    fn find_existing_model(&self, model_path: &str) -> Option<Arc<LlamaModel>> {
        let binding = self.model_cache.lock().unwrap();
        binding.get(model_path).map(Arc::clone)
    }

    fn get_or_create_model(&self, model_path: &str) -> Result<Arc<LlamaModel>> {
        let model_path = model_path.to_string();

        let model = match self.find_existing_model(&model_path) {
            Some(model) => model.clone(),
            None => {
                let model = self.create_new_model(&model_path, &self.new_model_options)?;
                let model = Arc::new(model);
                self.model_cache
                    .lock()
                    .unwrap()
                    .insert(model_path, model.clone());

                model
            }
        };

        Ok(model)
    }
}

impl ModelFactory for CachedModelFactory {
    fn create(&self, model_path: &str) -> Result<Arc<LlamaModel>> {
        self.get_or_create_model(model_path)
    }
}
