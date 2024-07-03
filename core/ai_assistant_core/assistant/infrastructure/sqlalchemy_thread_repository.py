from typing import Optional
from injector import inject
from langchain_openai_api_bridge.assistant import (
    ThreadRepository,
)
from openai.types.beta import Thread, ThreadDeleted
from sqlalchemy.orm import Session

from .thread_schema import ThreadModel


@inject
class SqlalchemyThreadRepository(ThreadRepository):

    def __init__(self, db: Session):
        self.db = db

    def create(
        self,
        metadata: Optional[object] = None,
    ) -> Thread:
        model = ThreadModel(metadata_=metadata)
        self.db.add(model)
        self.db.commit()
        self.db.refresh(model)

        return model.to_dto()

    def update(
        self,
        thread_id: str,
        metadata: Optional[object] = None,
    ) -> Thread:
        model = self.db.query(ThreadModel).filter(ThreadModel.id == thread_id).first()
        if model:
            model.metadata_ = metadata
            self.db.commit()
            self.db.refresh(model)

        return model.to_dto()

    def retreive(self, thread_id: str) -> Thread:
        model = self.db.query(ThreadModel).filter(ThreadModel.id == thread_id).first()
        if model is None:
            return None
        return model.to_dto()

    def delete(
        self,
        thread_id: str,
    ) -> ThreadDeleted:
        model = self.db.query(ThreadModel).filter(ThreadModel.id == thread_id).first()
        if model:
            self.db.delete(model)
            self.db.commit()

        return model
