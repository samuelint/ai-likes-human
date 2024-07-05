from injector import inject
from llama_cpp.server.settings import ModelSettings
from llama_cpp.server.app import LlamaProxy

from ai_assistant_core.llm.domain.local_model_service import LocalLLMModelService


class LlamaCppProxyFactory:
    @inject
    def __init__(self, local_model_service: LocalLLMModelService) -> None:
        self.local_model_service = local_model_service

    def create_model_settings(self, local_path: str, model_alias: str) -> ModelSettings:
        # All thoses values are hardcoded to work well on a M3 Pro MacBook pro
        #  and should be dynamically defined from user machine
        return ModelSettings(
            model=local_path,
            model_alias=model_alias,
            n_ctx=10240,
            n_batch=512,  # Should be between 1 and n_ctx, consider the amount of RAM.
            offload_kqv=True,  # Equivalent of f16_kv=True
            n_gpu_layers=-1,  # -1 is all on GPU
            verbose=False,
        )

    def create_llama_proxy(self) -> LlamaProxy:
        local_models = self.local_model_service.list()

        if len(local_models) <= 0:
            raise ValueError("No local model found")

        model_settings = [
            self.create_model_settings(
                local_path=model_info.local_path, model_alias=model_info.name
            )
            for model_info in local_models
        ]

        return LlamaProxy(models=model_settings)
