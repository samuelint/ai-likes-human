from typing import List, Optional

from injector import inject
from langchain_core.language_models import BaseChatModel
from langchain_core.messages import BaseMessage

from langchain_llamacpp_chat_model.proxy import LlamaProxyChatModel

from ai_assistant_core.llm.infrastructure.llamacpp_proxy_factory import (
    LlamaCppProxyFactory,
)
from langchain_openai_api_bridge.chat_model_adapter.llamacpp.llamacpp_openai_compatible_chat_model_adapter import (
    LlamacppOpenAICompatibleChatModelAdapter,
)

from ..domain.base_llm_factory import BaseLLMFactory


class LLamacppProxyOpenAICompatibleChatModel(LlamaProxyChatModel):

    adapter = LlamacppOpenAICompatibleChatModelAdapter()

    def _stream(self, messages: List[List[BaseMessage]], **kwargs):
        transformed_messages = self.adapter.to_openai_format_messages(messages)

        return super()._stream(messages=transformed_messages, **kwargs)

    def _astream(self, messages: List[List[BaseMessage]], **kwargs):
        transformed_messages = self.adapter.to_openai_format_messages(messages)

        return super()._astream(transformed_messages, **kwargs)

    def _generate(self, messages: List[List[BaseMessage]], **kwargs):
        transformed_messages = self.adapter.to_openai_format_messages(messages)

        return super().generate(transformed_messages, **kwargs)

    def _agenerate(self, messages: List[List[BaseMessage]], **kwargs):
        transformed_messages = self.adapter.to_openai_format_messages(messages)

        return super()._agenerate(transformed_messages, **kwargs)


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
        return LLamacppProxyOpenAICompatibleChatModel(
            llama_proxy=llama_proxy,
            model_name=model,
            max_tokens=max_tokens,
            temperature=temperature,
        )
