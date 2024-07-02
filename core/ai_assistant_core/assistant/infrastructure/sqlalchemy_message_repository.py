from injector import inject
from sqlalchemy.orm import Session
from langchain_openai_api_bridge.assistant import (
    MessageRepository,
)
from typing import Iterable, List, Literal, Optional, Union
from openai.types.beta.threads import Message, MessageDeleted, MessageContentPartParam
from openai.types.beta import thread_create_params
from openai.pagination import SyncCursorPage


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
        run_id: Optional[str] = None,
        metadata: Optional[dict] = {},
    ) -> Message:
        pass

    def create_many(
        self,
        thread_id: str,
        messages: List[thread_create_params.Message],
    ) -> Message:
        pass

    def list(
        self,
        thread_id: str,
    ) -> List[Message]:
        pass

    def listByPage(
        self,
        thread_id: str,
        after: str = None,
        before: str = None,
        limit: int = None,
        order: Literal["asc", "desc"] = None,
    ) -> SyncCursorPage[Message]:
        pass

    def retreive(self, message_id: str, thread_id: str) -> Message:
        pass

    def retreive_unique_by_run_id(
        self, run_id: str, thread_id: str
    ) -> Union[Message, None]:
        pass

    # The id is required for message delta, however, it's not necessary to hit the database
    # every time. The correlation id - run_id can be cached in this function
    def retreive_message_id_by_run_id(
        self, run_id: str, thread_id: str
    ) -> Union[str, None]:
        pass

    def update(self, message: Message) -> Message:
        pass

    def delete(self, message_id: str, thread_id: str) -> MessageDeleted:
        pass
