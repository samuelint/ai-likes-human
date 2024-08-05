from injector import inject
from langchain_core.tools import BaseTool, StructuredTool
from ai_assistant_core.extension.domain.base_extension_repository import (
    BaseExtensionRepository,
)
from ai_assistant_core.extension.infrastructure.pex_extension_inference_factory import (
    PexExtensionInferenceFactory,
)


@inject
class ExtensionAsToolFactory:
    def __init__(
        self,
        extension_repository: BaseExtensionRepository,
        extension_inference_service: PexExtensionInferenceFactory,
    ) -> None:
        self.extension_repository = extension_repository
        self.extension_inference_service = extension_inference_service

    def create(self, extension_name: str) -> BaseTool:
        runnable = self.extension_inference_service.create_as_runnable(
            name=extension_name
        )

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
