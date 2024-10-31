static LLAMA_BACKEND: tokio::sync::OnceCell<llama_cpp_2::llama_backend::LlamaBackend> =
    tokio::sync::OnceCell::const_new();