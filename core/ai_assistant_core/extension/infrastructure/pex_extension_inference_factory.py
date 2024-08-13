from injector import inject

from ai_assistant_core.extension.domain.inferable_extension import InferableExtension
from ai_assistant_core.extension.infrastructure.pex_extension_api_factory import (
    PexExtensionApiFactory,
)


@inject
class PexExtensionInferenceFactory:
    def __init__(
        self,
        extension_api_factory: PexExtensionApiFactory,
    ) -> None:
        self.extension_api_factory = extension_api_factory

    def create(
        self,
        extension_name: str,
        extension_llm_model: str,
    ) -> InferableExtension:
        api_service = self.extension_api_factory.create_from_extension_name(
            extension_name=extension_name
        )

        metadata = api_service.get_metadata()
        name = metadata["name"]
        description = metadata["description"]

        runnable = api_service.get_proxy_openai_chat_client(model=extension_llm_model)

        return InferableExtension(
            name=name,
            description=description,
            runnable=runnable,
        )
