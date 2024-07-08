from typing import Optional

from injector import inject
from ai_assistant_core.assistant.domain.user_info_service import UserInfo
from .prompt import build_system_prompt
from ...agent_factory import BaseAgentFactory
from langchain_core.language_models import BaseChatModel
from langchain_core.tools import BaseTool
from langgraph.graph.graph import CompiledGraph
from langgraph.prebuilt import create_react_agent


@inject
class DefaultAgentFactory(BaseAgentFactory):
    def __init__(self, tools: Optional[list[BaseTool]], user_info: UserInfo) -> None:
        self.tools = tools
        self.user_info = user_info

    def create(self, llm: BaseChatModel) -> CompiledGraph:
        user_name = self.user_info.name
        system_prompt = build_system_prompt(user_name=user_name)

        return create_react_agent(
            model=llm, tools=self.tools, messages_modifier=system_prompt
        )
