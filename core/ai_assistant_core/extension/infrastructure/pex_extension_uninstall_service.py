from injector import inject
from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto
from ai_assistant_core.extension.infrastructure.pex_extension_repository import (
    PexExtensionRepository,
)


@inject
class PexExtensionUninstallService:
    def __init__(self, repository: PexExtensionRepository) -> None:
        self.repository = repository

    def uninstall_by_name(self, name: str) -> ExtensionInfoDto:
        extension = self.repository.get_by_name(name=name)
        return self.uninstall(extension=extension)

    def uninstall(self, extension: ExtensionInfoDto) -> ExtensionInfoDto:
        return self.repository.delete(name=extension.name)