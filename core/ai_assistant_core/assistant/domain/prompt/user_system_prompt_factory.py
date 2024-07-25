from injector import inject
from .prompt import build_system_prompt
from ai_assistant_core.assistant.domain.user_info_service import UserInfo
from ai_assistant_core.assistant.domain.user_machine_info_service import (
    UserMachineInfoService,
)

from langgraph.graph.graph import CompiledGraph


@inject
class UserSystemPromptFactory:
    def __init__(
        self,
        user_info: UserInfo,
        user_machine_info_service: UserMachineInfoService,
    ) -> None:
        self.user_info = user_info
        self.user_machine_info_service = user_machine_info_service

    def create(self) -> CompiledGraph:
        user_name = self.user_info.name
        computer_info = self.user_machine_info_service.get_machine_info()
        return build_system_prompt(user_name=user_name, computer_info=computer_info)
