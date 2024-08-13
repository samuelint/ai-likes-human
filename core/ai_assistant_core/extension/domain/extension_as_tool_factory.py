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

    def create(self, extension_name: str, extension_llm_model: str) -> BaseTool:
        inferable = self.extension_inference_service.create(
            extension_name=extension_name,
            extension_llm_model=extension_llm_model,
        )
        runnable = inferable.runnable

        def call_extension(query: str) -> str:
            try:
                result = runnable.invoke(query)

                return result.content
            except Exception as e:
                return f"Error: {e}"

        tool_name = inferable.name.replace(" ", "_")
        tool_description = inferable.description

        return StructuredTool.from_function(
            func=call_extension,
            name=tool_name,
            description=tool_description,
        )
