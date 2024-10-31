const LOCAL_MODEL_PATH: &str = "~/.cache/lm-studio/models/lmstudio-community/Meta-Llama-3-8B-Instruct-GGUF/Meta-Llama-3-8B-Instruct-Q4_K_M.gguf";

pub fn get_tmp_model_path() -> String {
    shellexpand::tilde(LOCAL_MODEL_PATH).to_string()
}