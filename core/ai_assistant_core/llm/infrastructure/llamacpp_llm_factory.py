from typing import Optional

from injector import inject
from langchain_core.language_models import BaseChatModel

from ai_assistant_core.llm.domain.local_model_service import LocalLLMModelService
from langchain_llamacpp_chat_model import LlamaProxyChatModel

from ..domain.base_llm_factory import BaseLLMFactory
from llama_cpp.server.app import LlamaProxy


@inject
class LlamaCPPFactory(BaseLLMFactory):

    @inject
    def __init__(
        self,
        local_model_service: LocalLLMModelService,
        llama_proxy: LlamaProxy,
    ) -> None:
        super().__init__()
        self.local_model_service = local_model_service
        self.llama_proxy = llama_proxy

    def is_compatible(self, vendor: str) -> bool:
        return vendor.lower() == "local"

    def create(
        self,
        model: str,
        max_tokens: Optional[int] = None,
        temperature: Optional[float] = 0,
    ) -> BaseChatModel:
        return LlamaProxyChatModel(
            llama_proxy=self.llama_proxy,
            model_name=model,
            max_tokens=max_tokens,
            temperature=temperature,
        )
