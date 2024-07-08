from typing import Optional

from injector import inject
from langchain_core.language_models import BaseChatModel

from langchain_llamacpp_chat_model import LlamaProxyChatModel

from ai_assistant_core.llm.infrastructure.llamacpp_proxy_factory import (
    LlamaCppProxyFactory,
)

from ..domain.base_llm_factory import BaseLLMFactory


@inject
class LlamaCPPFactory(BaseLLMFactory):

    @inject
    def __init__(
        self,
        factory: LlamaCppProxyFactory,
    ) -> None:
        super().__init__()
        self.factory = factory

    def is_compatible(self, vendor: str) -> bool:
        return vendor.lower() == "local"

    def create(
        self,
        model: str,
        max_tokens: Optional[int] = None,
        temperature: Optional[float] = 0,
    ) -> BaseChatModel:
        llama_proxy = self.factory.get_llama_proxy()
        return LlamaProxyChatModel(
            llama_proxy=llama_proxy,
            model_name=model,
            max_tokens=max_tokens,
            temperature=temperature,
        )
