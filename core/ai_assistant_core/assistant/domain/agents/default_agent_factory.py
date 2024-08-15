from typing import Optional

from injector import inject
from ai_assistant_core.assistant.domain.prompt.user_system_prompt_factory import (
    UserSystemPromptFactory,
)
from langchain_core.language_models import BaseChatModel
from langchain_core.tools import BaseTool
from langgraph.graph.graph import CompiledGraph
from langgraph.prebuilt import create_react_agent


@inject
class DefaultAgentFactory:
    def __init__(
        self,
        tools: Optional[list[BaseTool]],
        prompt_factory: UserSystemPromptFactory,
    ) -> None:
        self.tools = tools
        self.prompt_factory = prompt_factory

    def create(self, llm: BaseChatModel) -> CompiledGraph:
        system_prompt = self.prompt_factory.create()

        return create_react_agent(
            model=llm,
            tools=self.tools,
            messages_modifier=system_prompt,
        )
