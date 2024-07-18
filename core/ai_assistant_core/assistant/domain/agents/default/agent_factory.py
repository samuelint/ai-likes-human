from typing import Optional

from injector import inject
from ai_assistant_core.assistant.domain.user_info_service import UserInfo
from ai_assistant_core.assistant.domain.user_machine_info_service import (
    UserMachineInfoService,
)
from .prompt import build_system_prompt
from ...agent_factory import BaseAgentFactory
from langchain_core.language_models import BaseChatModel
from langchain_core.tools import BaseTool
from langgraph.graph.graph import CompiledGraph
from langgraph.prebuilt import create_react_agent


@inject
class DefaultAgentFactory(BaseAgentFactory):
    def __init__(
        self,
        tools: Optional[list[BaseTool]],
        user_info: UserInfo,
        user_machine_info_service: UserMachineInfoService,
    ) -> None:
        self.tools = tools
        self.user_info = user_info
        self.user_machine_info_service = user_machine_info_service

    def create(self, llm: BaseChatModel) -> CompiledGraph:
        user_name = self.user_info.name
        computer_info = self.user_machine_info_service.get_machine_info()
        system_prompt = build_system_prompt(
            user_name=user_name, computer_info=computer_info
        )

        return create_react_agent(
            model=llm, tools=self.tools, messages_modifier=system_prompt
        )
