from injector import inject

from ai_assistant_core.extension.domain.extension_load_state import (
    ExtensionLoadStateDto,
)
from ai_assistant_core.extension.infrastructure.in_memory_loaded_extension_repository import (
    InMemoryLoadedExtensionRepository,
)
from ai_assistant_core.extension.infrastructure.pex_extension_metadata_api import (
    PexExtensionApi,
)


@inject
class PexExtensionApiFactory:
    def __init__(
        self,
        loaded_extensions_repository: InMemoryLoadedExtensionRepository,
    ) -> None:
        self.loaded_extensions_repository = loaded_extensions_repository

    def create(self, loaded_extension: ExtensionLoadStateDto) -> PexExtensionApi:
        return PexExtensionApi(uri=f"http://localhost:{loaded_extension.ipc_port}")

    def create_from_extension_name(self, extension_name: str) -> PexExtensionApi:
        loaded_extension = self.loaded_extensions_repository.get_by_name(
            name=extension_name
        )

        return self.create(loaded_extension=loaded_extension)
