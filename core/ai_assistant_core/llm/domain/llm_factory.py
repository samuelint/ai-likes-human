from typing import Optional
from injector import inject
from langchain_core.language_models import BaseChatModel
from ai_assistant_core.llm.domain.base_llm_factory import BaseLLMFactory


class LLMFactory:
    @inject
    def __init__(self, factories: list[BaseLLMFactory]) -> None:
        self.factories = factories

    def create_chat_model(
        self,
        vendor_model: str,
        max_tokens: Optional[int] = None,
        temperature: Optional[float] = None,
    ) -> BaseChatModel:
        vendor, model = self.__extract_vendor_model(vendor_model)

        for factory in self.factories:
            if factory.isCompatible(vendor):
                return factory.create(
                    model=model,
                    max_tokens=max_tokens,
                    temperature=temperature,
                )

        raise ValueError(f"Unknown LLM vendor: {vendor}").with_traceback(None)

    def __extract_vendor_model(self, vendor_model: str) -> tuple[str, str]:
        splitted = vendor_model.split(":")
        if len(splitted) <= 1:
            raise ValueError("model must be in the format vendor:model")

        return splitted
