from injector import inject
from sqlalchemy.orm import Session
from langchain_openai_api_bridge.assistant import (
    MessageRepository,
)
from typing import Iterable, List, Literal, Optional, Union
from openai.types.beta.threads import Message, MessageDeleted, MessageContentPartParam
from openai.types.beta.threads.message import Attachment

from openai.types.beta import thread_create_params
from openai.pagination import SyncCursorPage

from ai_assistant_core.assistant.infrastructure.thread_message_schema import (
    ThreadMessageModel,
)


@inject
class SqlalchemyMessageRepository(MessageRepository):
    def __init__(self, db: Session):
        self.db = db

    def create(
        self,
        thread_id: str,
        role: Literal["user", "assistant"],
        content: Union[str, Iterable[MessageContentPartParam]],
        status: Literal["in_progress", "incomplete", "completed"] = "completed",
        assistant_id: Optional[str] = None,
        attachments: Optional[List[Attachment]] = None,
        run_id: Optional[str] = None,
        metadata: Optional[dict] = {},
    ) -> Message:
        model = ThreadMessageModel.from_args(
            assistant_id=assistant_id,
            thread_id=thread_id,
            role=role,
            content=content,
            status=status,
            attachments=attachments,
            run_id=run_id,
            metadata=metadata,
        )
        self.db.add(model)
        self.db.commit()
        self.db.refresh(model)

        return model.to_dto()

    def create_many(
        self,
        thread_id: str,
        messages: List[thread_create_params.Message],
    ) -> Message:
        models = [
            ThreadMessageModel.from_args(
                thread_id=thread_id,
                role=msg["role"],
                content=msg["content"],
                attachments=msg.get("attachments", None),
                metadata=msg.get("metadata", None),
            )
            for msg in messages
        ]
        self.db.add_all(models)
        self.db.commit()

        [self.db.refresh(model) for model in models]
        return [model.to_dto() for model in models]

    def list(
        self,
        thread_id: str,
    ) -> List[Message]:
        result = (
            self.db.query(ThreadMessageModel)
            .filter_by(thread_id=thread_id)
            .order_by(ThreadMessageModel.created_at.asc())
            .all()
        )
        return [model.to_dto() for model in result]

    def listByPage(
        self,
        thread_id: str,
        after: str = None,
        before: str = None,
        limit: int = None,
        order: Literal["asc", "desc"] = None,
    ) -> SyncCursorPage[Message]:
        messages = self.list(thread_id=thread_id)

        return SyncCursorPage(data=messages)

    def retreive(self, message_id: str, thread_id: str) -> Union[Message, None]:
        result = (
            self.db.query(ThreadMessageModel)
            .filter_by(thread_id=thread_id, id=message_id)
            .first()
        )
        if result is None:
            return None
        return result.to_dto()

    def retreive_unique_by_run_id(
        self, run_id: str, thread_id: str
    ) -> Union[Message, None]:
        result = (
            self.db.query(ThreadMessageModel)
            .filter_by(thread_id=thread_id, run_id=run_id)
            .first()
        )
        if result is None:
            return None
        return result.to_dto()

    def retreive_message_id_by_run_id(
        self, run_id: str, thread_id: str
    ) -> Union[str, None]:
        result = self.retreive_unique_by_run_id(run_id=run_id, thread_id=thread_id)
        if result is None:
            return None
        return result.id

    def update(self, message: Message) -> Message:
        model_ = (
            self.db.query(ThreadMessageModel)
            .filter(ThreadMessageModel.id == message.id)
            .first()
        )
        if model_:
            model_to_change = ThreadMessageModel.from_dto(message)
            for attr in model_to_change.__dict__.keys():
                if attr != "_sa_instance_state":
                    setattr(model_, attr, getattr(model_to_change, attr))
            self.db.commit()
            self.db.refresh(model_)

        return model_.to_dto()

    def delete(self, message_id: str, thread_id: str) -> MessageDeleted:
        model_ = (
            self.db.query(ThreadMessageModel)
            .filter_by(thread_id=thread_id, id=message_id)
            .first()
        )
        if model_:
            self.db.delete(model_)
            self.db.commit()

        return model_

    def delete_with_thread_id(self, thread_id: str):
        self.db.query(ThreadMessageModel).filter(
            ThreadMessageModel.thread_id == thread_id
        ).delete(synchronize_session=False)
        self.db.commit()
