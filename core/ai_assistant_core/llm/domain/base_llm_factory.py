from abc import ABC, abstractmethod
from typing import Optional
from langchain_core.language_models import BaseChatModel


class BaseLLMFactory(ABC):

    @abstractmethod
    def isCompatible(self, vendor: str) -> bool:
        pass

    @abstractmethod
    def create(
        self,
        model: str,
        max_tokens: Optional[int] = None,
        temperature: Optional[float] = None,
        streaming: Optional[bool] = None,
    ) -> BaseChatModel:
        pass
