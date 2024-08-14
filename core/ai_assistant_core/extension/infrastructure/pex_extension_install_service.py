from typing import BinaryIO, Optional
from injector import inject

from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto

from ai_assistant_core.extension.infrastructure.pex_extension_load_service import (
    PexExtensionLoadService,
)
from ai_assistant_core.extension.infrastructure.pex_installed_extension_repository import (
    PexInstalledExtensionRepository,
)


@inject
class PexExtensionInstallService:
    def __init__(
        self,
        repository: PexInstalledExtensionRepository,
        load_service: PexExtensionLoadService,
    ) -> None:
        self.repository = repository
        self.load_service = load_service

    def find_by_name(self, name: str) -> Optional[ExtensionInfoDto]:
        return self.repository.find_by_name(name=name)

    def install(self, filename: str, file: BinaryIO) -> ExtensionInfoDto:
        extension_info = self.repository.upload(filename=filename, file=file)

        return extension_info

    def list(self) -> list[ExtensionInfoDto]:
        return self.repository.list()
