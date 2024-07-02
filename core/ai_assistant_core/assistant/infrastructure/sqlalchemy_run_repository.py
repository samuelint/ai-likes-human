from typing import List, Optional
from injector import inject
from sqlalchemy.orm import Session
from langchain_openai_api_bridge.assistant import (
    RunRepository,
)
from openai.types.beta.threads import Run
from openai.types.beta.threads.run import RequiredAction, RunStatus, AssistantTool


@inject
class SqlalchemyRunRepository(RunRepository):
    def __init__(self, db: Session):
        self.db = db

    def create(
        self,
        assistant_id: str,
        thread_id: str,
        model: str,
        status: RunStatus,
        instructions: str = "",
        required_action: Optional[RequiredAction] = None,
        tools: List[AssistantTool] = [],
        parallel_tool_calls: bool = True,
    ) -> Run:
        pass

    def update(self, run: Run) -> Run:
        pass

    def retreive(self, thread_id: str) -> Run:
        pass

    def delete(self, run: Optional[Run], run_id: Optional[str]) -> None:
        pass
