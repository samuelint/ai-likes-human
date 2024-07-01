from typing import Optional
from injector import inject
from langchain_anthropic import ChatAnthropic
from ai_assistant_core.llm.domain.api_key_service import ApiKeyService
from ..domain.base_llm_factory import BaseLLMFactory


class AnthropicLLMFactory(BaseLLMFactory):
    @inject
    def __init__(self, api_key_service: ApiKeyService) -> None:
        super().__init__()

        self.api_key_service = api_key_service

    def isCompatible(self, vendor: str) -> bool:
        return vendor.lower() == "anthropic"

    def create(
        self,
        model: str,
        max_tokens: Optional[int] = 1024,
        temperature: Optional[float] = 0.7,
        streaming: bool = True,
    ) -> ChatAnthropic:
        api_key = self.api_key_service.get_anthropic_api_key()

        return ChatAnthropic(
            model=model,
            max_tokens=max_tokens,
            temperature=temperature,
            api_key=api_key,
            streaming=streaming,
        )
