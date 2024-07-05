import os
from typing import Optional

from injector import inject
from ai_assistant_core.llm.domain.local_model_dto import (
    LLMModelFileDto,
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

        self._delete_local_files(deleted_index_item.local_files)

        return deleted_index_item

    def _delete_local_files(self, local_files: LLMModelFileDto) -> None:
        if local_files.q4_gguf_filepath is not None and os.path.exists(
            local_files.q4_gguf_filepath
        ):
            os.remove(local_files.q4_gguf_filepath)

        if local_files.fp16_gguf_filepath is not None and os.path.exists(
            local_files.fp16_gguf_filepath
        ):
            os.remove(local_files.fp16_gguf_filepath)
