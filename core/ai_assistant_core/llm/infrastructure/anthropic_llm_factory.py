from typing import Optional
from injector import inject
from langchain_anthropic import ChatAnthropic
from ai_assistant_core.llm.domain.api_key_service import ApiKeyService
from ..domain.base_llm_factory import BaseLLMFactory


@inject
class AnthropicLLMFactory(BaseLLMFactory):
    def __init__(self, api_key_service: ApiKeyService) -> None:
        super().__init__()

        self.api_key_service = api_key_service

    def is_compatible(self, vendor: str) -> bool:
        return vendor.lower() == "anthropic"

    def create(
        self,
        model: str,
        max_tokens: Optional[int] = None,
        temperature: Optional[float] = None,
        streaming: bool = True,
    ) -> ChatAnthropic:
        api_key = self.api_key_service.get_anthropic_api_key()

        client = ChatAnthropic(
            model=model,
            max_tokens=max_tokens or 2048,
            temperature=temperature or 0.7,
            api_key=api_key,
            streaming=streaming,
        )

        return client
