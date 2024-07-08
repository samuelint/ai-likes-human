from typing import Optional
from injector import inject
from sqlalchemy.orm import Session
from ai_assistant_core.llm.domain.local_model_dto import LLMModelIndex
from ai_assistant_core.llm.domain.local_model_index_repository import (
    LocalLLMModelIndexRepository,
)
from .local_model_index_schema import LocalModelIndexModel


class SqlAlchemyLocalLLMModelIndexRepository(LocalLLMModelIndexRepository):
    @inject
    def __init__(self, db: Session):
        self.db = db

    def read(self, model_name: str) -> Optional[LLMModelIndex]:
        model = (
            self.db.query(LocalModelIndexModel)
            .filter(LocalModelIndexModel.name == model_name)
            .first()
        )
        if model is None:
            return None
        return model.to_dto()

    def list(self, skip: int = 0, limit: int = 100) -> list[LLMModelIndex]:
        items = self.db.query(LocalModelIndexModel).offset(skip).limit(limit).all()

        return [item.to_dto() for item in items]

    def create(self, dto: LLMModelIndex) -> LLMModelIndex:
        model = LocalModelIndexModel.from_dto(dto)
        self.db.add(model)
        self.db.commit()
        self.db.refresh(model)

        return model.to_dto()

    def update(self, dto: LLMModelIndex) -> LLMModelIndex:
        model = (
            self.db.query(LocalModelIndexModel)
            .filter(LocalModelIndexModel.name == dto.name)
            .first()
        )
        if model:
            model_to_change = LocalModelIndexModel.from_dto(dto)
            for attr in model_to_change.__dict__.keys():
                if attr != "_sa_instance_state":
                    setattr(model, attr, getattr(model_to_change, attr))
            self.db.commit()
            self.db.refresh(model)

        return model.to_dto()

    def delete(self, model_name: str) -> LLMModelIndex:
        model = (
            self.db.query(LocalModelIndexModel)
            .filter(LocalModelIndexModel.name == model_name)
            .first()
        )
        if model:
            self.db.delete(model)
            self.db.commit()

        return model
