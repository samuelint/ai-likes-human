from typing import Self
from sqlalchemy import JSON, Column, String
from sqlalchemy.orm import mapped_column

from ai_assistant_core.infrastructure.sqlalchemy import Base
from ai_assistant_core.llm.domain.local_model_dto import (
    FileLLMModelIndex,
    HuggingFaceLLMModelIndex,
    LLMModelIndex,
)


class LocalModelIndexModel(Base):
    __tablename__ = "local_model_index"

    name = mapped_column(
        String,
        primary_key=True,
    )
    type = Column(String)
    local_files = Column(JSON, nullable=True)

    repo_id = Column(String, nullable=True)
    remote_files = Column(JSON, nullable=True)

    @staticmethod
    def from_dto(
        dto: LLMModelIndex,
    ) -> Self:
        if isinstance(dto, FileLLMModelIndex):
            return LocalModelIndexModel(
                name=dto.name,
                type=dto.type,
                local_files=dto.local_files,
            )
        elif isinstance(dto, HuggingFaceLLMModelIndex):
            return LocalModelIndexModel(
                name=dto.name,
                type=dto.type,
                repo_id=dto.repo_id,
                local_files=dto.local_files,
                remote_files=dto.remote_files,
            )
        else:
            raise ValueError(f"Unsupported LLMModelIndex type: {type(dto)}")

    def to_dto(self) -> LLMModelIndex:
        if self.type == "local":
            return FileLLMModelIndex(
                name=self.name,
                type=self.type,
                local_files=self.local_files,
            )
        elif self.type == "huggingface":
            return HuggingFaceLLMModelIndex(
                name=self.name,
                type=self.type,
                repo_id=self.repo_id,
                local_files=self.local_files,
                remote_files=self.remote_files,
            )
        else:
            raise ValueError(f"Unsupported model type: {self.type}")
