from typing import Optional

from injector import inject
from langchain_openai import ChatOpenAI

from ai_assistant_core.llm.domain.api_key_service import ApiKeyService
from ..domain.base_llm_factory import BaseLLMFactory


@inject
class OpenAILLMFactory(BaseLLMFactory):

    def __init__(self, api_key_service: ApiKeyService) -> None:
        super().__init__()

        self.api_key_service = api_key_service

    def is_compatible(self, vendor: str) -> bool:
        return vendor.lower() == "openai"

    def create(
        self,
        model: str,
        max_tokens: Optional[int] = None,
        temperature: Optional[float] = None,
        streaming: bool = True,
    ) -> ChatOpenAI:
        api_key = self.api_key_service.get_openai_api_key()

        return ChatOpenAI(
            model=model,
            max_tokens=max_tokens,
            temperature=temperature or 0.7,
            api_key=api_key,
            streaming=streaming,
        )
