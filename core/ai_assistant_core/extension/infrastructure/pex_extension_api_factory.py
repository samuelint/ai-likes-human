from injector import inject

from ai_assistant_core.extension.domain.extension_load_state import (
    ExtensionLoadStateDto,
)
from ai_assistant_core.extension.infrastructure.pex_extension_load_service import (
    PexExtensionLoadService,
)
from ai_assistant_core.extension.infrastructure.pex_extension_metadata_api import (
    PexExtensionApi,
)


@inject
class PexExtensionApiFactory:
    def __init__(
        self,
        extension_load_service: PexExtensionLoadService,
    ) -> None:
        self.extension_load_service = extension_load_service

    def create(self, loaded_extension: ExtensionLoadStateDto) -> PexExtensionApi:
        return PexExtensionApi(uri=f"http://localhost:{loaded_extension.ipc_port}")

    def create_from_extension_name(self, extension_name: str) -> PexExtensionApi:
        loaded_extension = self.extension_load_service.assert_loaded_extension(
            extension_name=extension_name
        )

        return self.create(loaded_extension=loaded_extension)
