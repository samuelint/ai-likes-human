import os
from typing import Optional

from injector import inject
from ai_assistant_core.llm.domain.local_model_dto import (
    LLMModelIndex,
)
from .local_model_index_repository import (
    LocalLLMModelIndexRepository,
)


class LocalLLMModelService:
    @inject
    def __init__(self, index_repository: LocalLLMModelIndexRepository):
        self.index_repository = index_repository

    def find(self, model_name: str) -> Optional[LLMModelIndex]:
        return self.index_repository.read(model_name=model_name)

    def list(self) -> list[LLMModelIndex]:
        return self.index_repository.list()

    def delete(self, id: str) -> None:
        deleted_index_item = self.index_repository.delete(model_name=id)

        self._delete_local_files(deleted_index_item)

        return deleted_index_item

    def _delete_local_files(self, index: LLMModelIndex) -> None:
        if index.local_path is not None and os.path.exists(index.local_path):
            os.remove(index.local_path)
