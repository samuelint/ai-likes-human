from injector import inject

from ai_assistant_core.extension.domain.base_extension_repository import (
    BaseExtensionRepository,
)
from ai_assistant_core.extension.domain.base_extension_service import (
    BaseExtensionService,
)
from ai_assistant_core.extension.domain.extension_as_tool_factory import (
    ExtensionAsToolFactory,
)


from langchain_core.language_models import BaseChatModel
from langchain_core.runnables import Runnable
from langgraph.prebuilt import create_react_agent


@inject
class ExtensionAgentFactory:
    def __init__(
        self,
        extension_repository: BaseExtensionRepository,
        extension_service: BaseExtensionService,
        extension_as_tool_factory: ExtensionAsToolFactory,
    ) -> None:
        self.extension_repository = extension_repository
        self.extension_service = extension_service
        self.extension_as_tool_factory = extension_as_tool_factory

    def is_assistant_an_extension(self, assistant_id: str) -> bool:
        extension = self.extension_repository.find_by_name(name=assistant_id)
        if extension is None:
            return False
        return True

    def create(self, assistant_id: str, llm: BaseChatModel) -> Runnable:
        extension_info = self.extension_repository.get_by_name(name=assistant_id)
        extension = self.extension_service.load(extension=extension_info)
        tool = self.extension_as_tool_factory.create(extension=extension, llm=llm)

        return create_react_agent(
            model=llm,
            tools=[tool],
            messages_modifier=f'No matter the input, call the tool "{tool.description}"',
        )
