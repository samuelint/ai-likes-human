from base_assistant_extension import BaseExtension
from injector import inject
from ai_assistant_core.extension.domain.base_extension_install_service import (
    BaseExtensionInstallService,
)
from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto
from ai_assistant_core.extension.infrastructure.whl_extension_loader import (
    WhlExtensionLoader,
)


@inject
class ExtensionLoadService:
    def __init__(self, install_service: BaseExtensionInstallService) -> None:
        self.install_service = install_service

    def load(self, extension: ExtensionInfoDto) -> BaseExtension:
        if not self.install_service.is_installed(extension=extension):
            self.install_service.install(extension=extension)

        loader = WhlExtensionLoader(wheel_path=extension.uri)

        return loader.load()
