from base_assistant_extension import BaseExtension
from injector import inject
from langchain_core.tools import BaseTool, StructuredTool
from langchain_core.language_models import BaseChatModel
from ai_assistant_core.extension.domain.base_extension_repository import (
    BaseExtensionRepository,
)
from ai_assistant_core.extension.domain.base_extension_install_service import (
    BaseExtensionInstallService,
)


@inject
class ExtensionAsToolFactory:
    def __init__(
        self,
        extension_repository: BaseExtensionRepository,
        extension_service: BaseExtensionInstallService,
    ) -> None:
        self.extension_repository = extension_repository
        self.extension_service = extension_service

    def create(self, extension: BaseExtension, llm: BaseChatModel) -> BaseTool:
        runnable = extension.create_runnable(llm=llm)

        def call_extension(query: str) -> str:
            result = runnable.invoke(query)

            return result.content

        tool_name = extension.name().replace(" ", "_")
        tool_description = extension.description()

        return StructuredTool.from_function(
            func=call_extension,
            name=tool_name,
            description=tool_description,
        )
