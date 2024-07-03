import time
from typing import Iterable, List, Literal, Optional, Self, Type, Union
import uuid
from openai import BaseModel
from sqlalchemy import Column, ForeignKey, Integer, String, JSON
from sqlalchemy.orm import mapped_column
from ai_assistant_core.infrastructure.sqlalchemy import Base
from openai.types.beta.threads.message import Message, MessageContent, Attachment
from openai.types.beta.threads import (
    MessageContentPartParam,
    TextContentBlock,
    ImageFileContentBlock,
    ImageURLContentBlock,
    Text,
)


# use langchain_openai_bridge version when released
def create_message_content(
    content: Union[str, Iterable[MessageContentPartParam]] = ""
) -> List[MessageContent]:
    if isinstance(content, str):
        inner_content = [
            TextContentBlock(text=Text(value=content, annotations=[]), type="text")
        ]
    else:
        inner_content = content

    return inner_content


# use langchain_openai_bridge version when released
def deserialize_message_content(data: dict) -> MessageContent:
    type_to_model: dict[str, Type[BaseModel]] = {
        "image_file": ImageFileContentBlock,
        "image_url": ImageURLContentBlock,
        "text": TextContentBlock,
    }

    content_type = data["type"]
    model_cls = type_to_model[content_type]
    return model_cls.parse_obj(data)


class ThreadMessageModel(Base):
    __tablename__ = "openai_thread_message"

    id = mapped_column(
        String(36),
        primary_key=True,
        default=lambda: str(uuid.uuid4().hex)[:36],
    )
    assistant_id = Column(String, nullable=True, index=True)
    attachments = Column(JSON, nullable=True)
    content = Column(JSON)
    created_at = Column(Integer, default=lambda: int(time.time()))
    metadata_ = Column("metadata", JSON, nullable=True)
    role = Column(String)
    run_id = mapped_column(
        String, ForeignKey("openai_run.id"), nullable=True, index=True
    )
    status = Column(String)
    thread_id = mapped_column(String, ForeignKey("openai_thread.id"), index=True)

    @staticmethod
    def from_dto(
        message: Message,
    ) -> Self:
        return ThreadMessageModel.from_args(
            thread_id=message.thread_id,
            role=message.role,
            content=message.content,
            status=message.status,
            assistant_id=message.assistant_id,
            attachments=message.attachments,
            run_id=message.run_id,
            metadata=message.metadata,
        )

    @staticmethod
    def from_args(
        thread_id: str,
        role: Literal["user", "assistant"],
        content: Union[str, Iterable[MessageContentPartParam]],
        status: Literal["in_progress", "incomplete", "completed"] = "completed",
        assistant_id: Optional[str] = None,
        attachments: Optional[List[Attachment]] = None,
        run_id: Optional[str] = None,
        metadata: Optional[dict] = {},
    ) -> Self:

        inner_content = [
            block.dict() for block in create_message_content(content=content)
        ]

        return ThreadMessageModel(
            assistant_id=assistant_id,
            thread_id=thread_id,
            status=status,
            role=role,
            content=inner_content,
            attachments=attachments,
            run_id=run_id,
            metadata=metadata,
        )

    def to_dto(self) -> Message:
        deserialized_content = [
            deserialize_message_content(block) for block in self.content
        ]
        return Message(
            id=self.id,
            assistant_id=self.assistant_id,
            # attachments=[Attachment(**c) for c in self.attachments],
            content=deserialized_content,
            created_at=self.created_at,
            metadata=self.metadata_,
            role=self.role,
            run_id=self.run_id,
            status=self.status,
            thread_id=self.thread_id,
            object="thread.message",
        )
