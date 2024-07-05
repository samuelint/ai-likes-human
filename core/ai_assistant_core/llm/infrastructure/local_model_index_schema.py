from typing import Self
from sqlalchemy import Column, String
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
    local_path = Column(String, nullable=True)

    repo_id = Column(String, nullable=True)
    remote_path = Column(String, nullable=True)

    @staticmethod
    def from_dto(
        dto: LLMModelIndex,
    ) -> Self:
        if isinstance(dto, FileLLMModelIndex):
            return LocalModelIndexModel(
                name=dto.name,
                type=dto.type,
                local_path=dto.local_path,
            )
        elif isinstance(dto, HuggingFaceLLMModelIndex):
            return LocalModelIndexModel(
                name=dto.name,
                type=dto.type,
                repo_id=dto.repo_id,
                local_path=dto.local_path,
                remote_path=dto.remote_path,
            )
        else:
            raise ValueError(f"Unsupported LLMModelIndex type: {type(dto)}")

    def to_dto(self) -> LLMModelIndex:
        if self.type == "local":
            return FileLLMModelIndex(
                name=self.name,
                type=self.type,
                local_path=self.local_path,
            )
        elif self.type == "huggingface":
            return HuggingFaceLLMModelIndex(
                name=self.name,
                type=self.type,
                repo_id=self.repo_id,
                local_path=self.local_path,
                remote_path=self.remote_path,
            )
        else:
            raise ValueError(f"Unsupported model type: {self.type}")
