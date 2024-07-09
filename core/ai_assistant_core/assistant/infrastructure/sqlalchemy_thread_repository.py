from typing import Literal, Optional
from injector import inject
from langchain_openai_api_bridge.assistant import (
    ThreadRepository,
)
from openai.types.beta import Thread, ThreadDeleted
from openai.pagination import SyncCursorPage
from sqlalchemy.orm import Session

from .thread_schema import ThreadModel


@inject
class SqlalchemyThreadRepository(ThreadRepository):

    def __init__(self, db: Session):
        self.db = db

    def create(
        self,
        metadata: Optional[object] = None,
        created_at: Optional[int] = None,
    ) -> Thread:
        model = ThreadModel(metadata_=metadata, created_at=created_at)
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

    def list(
        self,
        after: str = None,
        before: str = None,
        limit: int = None,
        order: Literal["asc", "desc"] = None,
    ) -> SyncCursorPage[Thread]:
        query = self.db.query(ThreadModel)
        if after:
            query = query.filter(ThreadModel.id > after)
        if before:
            query = query.filter(ThreadModel.id < before)
        if order == "desc":
            query = query.order_by(ThreadModel.created_at.desc())
        else:
            query = query.order_by(ThreadModel.created_at.asc())
        if limit is not None:
            query = query.limit(limit)

        models = query.all()
        threads = [model.to_dto() for model in models]

        return SyncCursorPage(
            data=threads,
            order=order,
            next_after=models[-1].created_at if models else None,
            next_before=models[0].created_at if models else None,
        )

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
