from injector import inject

from ai_assistant_core.extension.domain.base_extension_inference_factory import (
    BaseExtensionInferenceFactory,
)
from ai_assistant_core.extension.domain.inferable_extension import InferableExtension
from ai_assistant_core.extension.infrastructure.pex_extension_api_factory import (
    PexExtensionApiFactory,
)
from ai_assistant_core.extension.infrastructure.pex_extension_load_service import (
    PexExtensionLoadService,
)


@inject
class PexExtensionInferenceFactory(BaseExtensionInferenceFactory):
    def __init__(
        self,
        extension_api_factory: PexExtensionApiFactory,
        load_service: PexExtensionLoadService,
    ) -> None:
        self.extension_api_factory = extension_api_factory
        self.load_service = load_service

    def create(
        self,
        extension_name: str,
        extension_llm_model: str,
    ) -> InferableExtension:
        self.load_service.assert_loaded_extension(extension_name=extension_name)
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
