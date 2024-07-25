from base_assistant_extension import BaseExtension
from injector import inject
from ai_assistant_core.extension.domain.base_extension_repository import (
    BaseExtensionRepository,
)
from ai_assistant_core.extension.domain.base_extension_service import (
    BaseExtensionService,
)
from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto

from ai_assistant_core.extension.infrastructure.whl_extension_loader import (
    WhlExtensionLoader,
)


@inject
class WhlExtensionService(BaseExtensionService):
    def __init__(self, repository: BaseExtensionRepository) -> None:
        self.repository = repository

    def load(self, extension: ExtensionInfoDto) -> BaseExtension:
        if not self.is_installed(extension=extension):
            self.install(extension=extension)

        loader = WhlExtensionLoader(wheel_path=extension.uri)

        return loader.load()

    def list_installed(self) -> list[ExtensionInfoDto]:
        available_extensions = self.repository.list_available()

        installed_extensions: list[ExtensionInfoDto] = []

        for extension in available_extensions:
            if self.is_installed(extension):
                installed_extensions.append(extension)

        return installed_extensions

    def install(self, extension: ExtensionInfoDto):
        loader = WhlExtensionLoader(wheel_path=extension.uri)
        loader.install()

    def uninstall(self, extension: ExtensionInfoDto):
        loader = WhlExtensionLoader(wheel_path=extension.uri)
        loader.uninstall()

    def is_installed(self, extension: ExtensionInfoDto) -> bool:
        loader = WhlExtensionLoader(wheel_path=extension.uri)
        return loader.is_installed()
