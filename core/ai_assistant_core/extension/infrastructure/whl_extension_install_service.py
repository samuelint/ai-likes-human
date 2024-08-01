from typing import BinaryIO, Optional
from injector import inject
from ai_assistant_core.extension.domain.base_extension_repository import (
    BaseExtensionRepository,
)
from ai_assistant_core.extension.domain.base_extension_install_service import (
    BaseExtensionInstallService,
)
from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto

from ai_assistant_core.extension.infrastructure.whl_extension_loader import (
    WhlExtensionLoader,
)


@inject
class WhlExtensionInstallService(BaseExtensionInstallService):
    def __init__(self, repository: BaseExtensionRepository) -> None:
        self.repository = repository

    def find_by_name(self, name: str) -> Optional[ExtensionInfoDto]:
        return self.repository.find_by_name(name=name)

    def upload(self, filename: str, file: BinaryIO) -> list[ExtensionInfoDto]:
        return self.repository.upload(filename=filename, file=file)

    def list_installed(self) -> list[ExtensionInfoDto]:
        return self.repository.list_available()

    def install(self, extension: ExtensionInfoDto):
        loader = WhlExtensionLoader(wheel_path=extension.uri)
        loader.install()

    def uninstall_by_name(self, name: str) -> ExtensionInfoDto:
        extension = self.repository.get_by_name(name=name)
        return self.uninstall(extension=extension)

    def uninstall(self, extension: ExtensionInfoDto) -> ExtensionInfoDto:
        loader = WhlExtensionLoader(wheel_path=extension.uri)
        loader.uninstall()

        return self.repository.delete(name=extension.name)

    def is_installed(self, extension: ExtensionInfoDto) -> bool:
        loader = WhlExtensionLoader(wheel_path=extension.uri)
        return loader.is_installed()
