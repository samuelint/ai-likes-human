import time
import uuid
from sqlalchemy import Column, Integer, String, JSON

from ai_assistant_core.infrastructure.sqlalchemy import Base
from openai.types.beta import Thread


class ThreadModel(Base):
    __tablename__ = "openai_thread"

    id = Column(
        String(36),
        primary_key=True,
        default=lambda: str(uuid.uuid4().hex)[:36],
    )
    created_at = Column(Integer, default=lambda: int(time.time()))
    metadata_ = Column("metadata", JSON, nullable=True)
    object = Column(String, default="thread")

    def to_dto(self) -> Thread:
        return Thread(
            id=self.id,
            created_at=self.created_at,
            metadata=self.metadata_,
            object=self.object,
        )
