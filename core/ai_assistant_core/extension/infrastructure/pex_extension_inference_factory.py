from injector import inject

from ai_assistant_core.extension.domain.extension_load_state import (
    ExtensionLoadStateDto,
)
from ai_assistant_core.extension.domain.inferable_extension import InferableExtension
from ai_assistant_core.extension.infrastructure.pex_extension_load_service import (
    PexExtensionLoadService,
)
from ai_assistant_core.extension.infrastructure.pex_extension_metadata_api import (
    PexExtensionApi,
)


@inject
class PexExtensionInferenceFactory:
    def __init__(
        self,
        extension_load_service: PexExtensionLoadService,
    ) -> None:
        self.extension_load_service = extension_load_service

    def create(self, extension_name: str) -> InferableExtension:
        loaded_extension = self.extension_load_service.assert_loaded_extension(
            extension_name=extension_name
        )
        api_service = self.create_api(loaded_extension=loaded_extension)

        metadata = api_service.get_metadata()
        name = metadata["name"]
        description = metadata["description"]

        runnable = api_service.get_proxy_openai_chat_client(model="gpt-4o-mini")

        return InferableExtension(
            name=name,
            description=description,
            runnable=runnable,
        )

    def create_api(self, loaded_extension: ExtensionLoadStateDto) -> PexExtensionApi:
        return PexExtensionApi(uri=f"http://localhost:{loaded_extension.ipc_port}")
