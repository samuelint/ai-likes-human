import time
from typing import List, Optional
import uuid
from sqlalchemy import Boolean, Column, Integer, Float, String, JSON, ForeignKey
from sqlalchemy.orm import mapped_column

from ai_assistant_core.infrastructure.sqlalchemy import Base
from openai.types.beta.threads import Run
from openai.types.beta.threads.run import RequiredAction, RunStatus, AssistantTool


class RunModel(Base):
    __tablename__ = "openai_run"

    # Not all fields from OpenAI Run object are implemented
    id = mapped_column(
        String(36),
        primary_key=True,
        default=lambda: str(uuid.uuid4().hex)[:36],
    )
    created_at = Column(Integer, default=lambda: int(time.time()))
    assistant_id = Column(String)
    thread_id = mapped_column(String, ForeignKey("openai_thread.id"))
    model = Column(String)
    status = Column(String)
    instructions = Column(String)
    parallel_tool_calls = Column(Boolean, default=True)
    required_action = Column(JSON, nullable=True)
    temperature = Column(Float, nullable=True)
    top_p = Column(Float, nullable=True)
    metadata_ = Column("metadata", JSON, nullable=True)

    def from_dto(
        assistant_id: str,
        thread_id: str,
        model: str,
        status: RunStatus,
        instructions: str = "",
        required_action: Optional[RequiredAction] = None,
        parallel_tool_calls: bool = True,
        tools: List[AssistantTool] = [],  # Tools are unsuppored for now
        metadata: Optional[object] = None,
        temperature: Optional[float] = None,
        top_p: Optional[float] = None,
    ) -> "RunModel":
        return RunModel(
            assistant_id=assistant_id,
            thread_id=thread_id,
            model=model,
            status=status,
            instructions=instructions,
            parallel_tool_calls=parallel_tool_calls or True,
            required_action=(
                RequiredAction.parse_raw(required_action) if required_action else None
            ),
            # tools=[],
            temperature=temperature,
            top_p=top_p,
            metadata_=metadata,
        )

    def to_dto(self) -> Run:
        return Run(
            id=self.id,
            assistant_id=self.assistant_id,
            created_at=self.created_at,
            instructions=self.instructions,
            model=self.model,
            object="thread.run",
            parallel_tool_calls=self.parallel_tool_calls or True,
            required_action=(
                RequiredAction.parse_raw(self.required_action)
                if self.required_action
                else None
            ),
            status=self.status,
            thread_id=self.thread_id,
            tools=[],
            temperature=self.temperature,
            top_p=self.top_p,
            metadata=self.metadata_,
        )
