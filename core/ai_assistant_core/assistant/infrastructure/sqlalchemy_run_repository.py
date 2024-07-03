from typing import List, Optional
from injector import inject
from sqlalchemy.orm import Session
from langchain_openai_api_bridge.assistant import (
    RunRepository,
)
from openai.types.beta.threads import Run
from openai.types.beta.threads.run import RequiredAction, RunStatus, AssistantTool

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

    def delete(self, run: Optional[Run], run_id: Optional[str]) -> None:
        id = run.id if run else run_id
        model_ = self.db.query(RunModel).filter(RunModel.id == id).first()
        if model_:
            self.db.delete(model_)
            self.db.commit()

        return model_
