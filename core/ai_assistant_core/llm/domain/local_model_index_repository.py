from abc import abstractmethod
from typing import Optional
from ai_assistant_core.llm.domain.local_model_dto import (
    LLMModelIndex,
)


class LocalLLMModelIndexRepository:
    @abstractmethod
    def read(self, model_name: str) -> Optional[LLMModelIndex]:
        pass

    @abstractmethod
    def list(self, skip: int = 0, limit: int = 100) -> list[LLMModelIndex]:
        pass

    @abstractmethod
    def create(self, dto: LLMModelIndex) -> LLMModelIndex:
        pass

    @abstractmethod
    def update(self, dto: LLMModelIndex) -> LLMModelIndex:
        pass

    @abstractmethod
    def delete(self, model_name: str) -> LLMModelIndex:
        pass
