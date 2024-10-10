use async_llama_cpp::options::ModelOptions;

const LOCAL_MODEL_PATH: &str = "~/.cache/lm-studio/models/lmstudio-community/Meta-Llama-3-8B-Instruct-GGUF/Meta-Llama-3-8B-Instruct-Q4_K_M.gguf";

fn get_test_model_path() -> String {
    shellexpand::tilde(LOCAL_MODEL_PATH).to_string()
}

#[tokio::test]
async fn test_invoke() {
    let model_gguf_path = get_test_model_path();

    let instance = async_llama_cpp::Builder::create()
        .unwrap()
        .with_model(
            "llama-3-8b",
            &model_gguf_path,
            ModelOptions {
                n_gpu_layers: 1000,
                ..Default::default()
            },
        )
        .unwrap()
        .build();

    let result = instance
        .invoke("llama-3-8b", "Tell me a joke about turtles.")
        .await
        .unwrap();

    assert!(result.len() > 0);
}
