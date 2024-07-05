from llama_cpp.server.settings import ModelSettings


class LlamaCppModelSettingsFactory:

    def create(self, local_path: str, model_alias: str) -> ModelSettings:
        # All thoses values are hardcoded to work well on a M3 Pro MacBook pro
        #  and should be dynamically defined from user machine
        return ModelSettings(
            model=local_path,
            model_alias=model_alias,
            n_ctx=10240,
            n_batch=512,  # Should be between 1 and n_ctx, consider the amount of RAM.
            offload_kqv=True,  # Equivalent of f16_kv=True
            n_gpu_layers=-1,  # -1 is all on GPU
        )
