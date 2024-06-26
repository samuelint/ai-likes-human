from typing import Optional
from langchain_core.language_models import BaseChatModel
from langchain_openai import ChatOpenAI
from langchain_anthropic import ChatAnthropic
from pydantic import SecretStr


class LLMProviderFactory:
    def __init__(self) -> None:
        pass

    def create(
        self,
        vendor_model: str,
        max_tokens: Optional[int] = None,
        temperature: float = 0.7,
        api_key: Optional[SecretStr] = None,
    ) -> BaseChatModel:
        vendor, model = self.__extract_vendor_model(vendor_model)

        match vendor:
            case "openai":
                return ChatOpenAI(
                    model=model,
                    max_tokens=max_tokens,
                    temperature=temperature,
                    api_key=api_key,
                    streaming=True,
                )
            case "anthropic":
                return ChatAnthropic(
                    model=model,
                    max_tokens=max_tokens or 1024,
                    temperature=temperature,
                    api_key=api_key,
                    streaming=True,
                )
            case _:
                raise ValueError(f"Unknown LLM vendor: {vendor}")

    def __extract_vendor_model(self, vendor_model: str) -> tuple[str, str]:
        splitted = vendor_model.split(":")
        if len(splitted) <= 1:
            raise ValueError("model must be in the format vendor:model")

        return splitted
