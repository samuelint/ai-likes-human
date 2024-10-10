use std::sync::Arc;

use llama_cpp_2::llama_backend::LlamaBackend;
use local_inference::llamacpp::{
    cached_model_factory::CachedModelFactory,
    llama_cpp::AsyncLLamaCPP,
    model_context_factory::ModelContextFactoryImpl,
    options::{ContextOptions, ModelOptions, RunOptions},
};

const LOCAL_MODEL_PATH: &str = "~/.cache/lm-studio/models/lmstudio-community/Meta-Llama-3-8B-Instruct-GGUF/Meta-Llama-3-8B-Instruct-Q4_K_M.gguf";

fn get_test_model_path() -> String {
    shellexpand::tilde(LOCAL_MODEL_PATH).to_string()
}

#[tokio::test]
async fn test_invoke() {
    let model_gguf_path = get_test_model_path();
    let backend = Arc::new(LlamaBackend::init().unwrap());

    let model_options = ModelOptions {
        n_gpu_layers: 1000,
        ..Default::default()
    };

    let context_options = ContextOptions {
        ..Default::default()
    };

    let instance = AsyncLLamaCPP::new(
        Arc::new(CachedModelFactory::new(backend.clone(), model_options)),
        Arc::new(ModelContextFactoryImpl::new(
            backend.clone(),
            context_options,
        )),
    );

    let run_options = RunOptions {
        ..Default::default()
    };

    let result = instance
        .invoke(
            &model_gguf_path,
            &run_options,
            "Tell me a joke about turtles.",
        )
        .await
        .unwrap();

    assert!(result.len() > 0);
}
