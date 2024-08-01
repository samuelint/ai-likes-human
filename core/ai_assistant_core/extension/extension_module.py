from injector import Binder, Module, provider, singleton

from ai_assistant_core.app_configuration import AppConfiguration
from ai_assistant_core.extension.domain.base_extension_repository import (
    BaseExtensionRepository,
)
from ai_assistant_core.extension.domain.base_extension_install_service import (
    BaseExtensionInstallService,
)
from ai_assistant_core.extension.infrastructure.whl_extension_repository import (
    WhlExtensionRepository,
)
from ai_assistant_core.extension.infrastructure.whl_extension_install_service import (
    WhlExtensionInstallService,
)


class ExtensionModule(Module):
    def configure(self, binder: Binder):
        binder.bind(BaseExtensionInstallService, to=WhlExtensionInstallService)

    @singleton
    @provider
    def provide_whl_extension_repository(
        self, configuration: AppConfiguration
    ) -> WhlExtensionRepository:
        extensions_directory = configuration.extensions_directory
        return WhlExtensionRepository(extensions_directory=extensions_directory)

    @singleton
    @provider
    def provide_base_extension_repository(
        self, whl_extension_repository: WhlExtensionRepository
    ) -> BaseExtensionRepository:
        return whl_extension_repository
