from typing import List, Literal, Optional
from injector import inject
from sqlalchemy.orm import Session
from langchain_openai_api_bridge.assistant import (
    RunRepository,
)
from openai.types.beta.threads import Run
from openai.types.beta.threads.run import RequiredAction, RunStatus, AssistantTool
from openai.pagination import SyncCursorPage
from ai_assistant_core.assistant.infrastructure.run_schema import RunModel


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
        metadata: Optional[object] = None,
        temperature: Optional[float] = None,
        top_p: Optional[float] = None,
    ) -> Run:
        model_ = RunModel.from_args(
            assistant_id=assistant_id,
            thread_id=thread_id,
            status=status,
            instructions=instructions,
            required_action=required_action,
            tools=tools,
            parallel_tool_calls=parallel_tool_calls,
            model=model,
            metadata=metadata,
            temperature=temperature,
            top_p=top_p,
        )
        self.db.add(model_)
        self.db.commit()
        self.db.refresh(model_)

        return model_.to_dto()

    def update(self, run: Run) -> Run:
        model_ = self.db.query(RunModel).filter(RunModel.id == run.id).first()
        if model_:
            model_to_change = RunModel.from_dto(run)
            for attr in model_to_change.__dict__.keys():
                if attr != "_sa_instance_state":
                    setattr(model_, attr, getattr(model_to_change, attr))
            self.db.commit()
            self.db.refresh(model_)

        return model_.to_dto()

    def retreive(self, run_id: str) -> Run:
        model_ = self.db.query(RunModel).filter(RunModel.id == run_id).first()
        if model_ is None:
            return None
        return model_.to_dto()

    def list(
        self,
        thread_id: str,
    ) -> Run:
        result = (
            self.db.query(RunModel)
            .filter_by(thread_id=thread_id)
            .order_by(RunModel.created_at.asc())
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
    ) -> SyncCursorPage[Run]:
        runs = self.list(thread_id=thread_id)

        return SyncCursorPage(data=runs)

    def delete(self, run: Optional[Run], run_id: Optional[str]) -> None:
        id = run.id if run else run_id
        model_ = self.db.query(RunModel).filter(RunModel.id == id).first()
        if model_:
            self.db.delete(model_)
            self.db.commit()

        return model_

    def delete_with_thread_id(self, thread_id: str) -> None:
        self.db.query(RunModel).filter(RunModel.thread_id == thread_id).delete(
            synchronize_session=False
        )
        self.db.commit()
