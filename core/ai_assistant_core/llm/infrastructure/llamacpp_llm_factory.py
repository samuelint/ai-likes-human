from typing import Optional

from injector import inject, singleton
from langchain_openai import ChatOpenAI
from llama_cpp.server.app import create_app
from llama_cpp.server.settings import ServerSettings

from ai_assistant_core.llm.domain.local_model_service import LocalLLMModelService
from ai_assistant_core.llm.infrastructure.llamacpp_model_settings_factory import (
    LlamaCppModelSettingsFactory,
)
from ..domain.base_llm_factory import BaseLLMFactory
from fastapi.testclient import TestClient


@inject
@singleton
class LlamaCPPFactory(BaseLLMFactory):

    @inject
    def __init__(
        self,
        local_model_service: LocalLLMModelService,
        model_settings_factory: LlamaCppModelSettingsFactory,
    ) -> None:
        super().__init__()
        self.local_model_service = local_model_service
        self.model_settings_factory = model_settings_factory
        self.llamacpp_app = None

    def is_compatible(self, vendor: str) -> bool:
        return vendor.lower() == "local"

    def create(
        self,
        model: str,
        max_tokens: Optional[int] = None,
        temperature: Optional[float] = 0.7,
        streaming: bool = True,
    ) -> ChatOpenAI:
        self.app = self._get_llamacpp_singleton_app()

        return ChatOpenAI(
            http_client=TestClient(self.app),
            model=model,
            max_tokens=max_tokens,
            temperature=temperature,
            streaming=streaming,
        )

    def _get_llamacpp_singleton_app(self):
        if self.llamacpp_app is not None:
            return self.llamacpp_app

        local_models = self.local_model_service.list()

        if len(local_models) <= 0:
            raise ValueError("No local model found")

        model_settings = [
            self.model_settings_factory.create(
                local_files=model_info.local_files, model_alias=model_info.name
            )
            for model_info in local_models
        ]

        self.llamacpp_app = create_app(
            server_settings=ServerSettings(host="localhost"),
            model_settings=model_settings,
        )

        return self.llamacpp_app
