from injector import Binder, Module, provider, singleton

from ai_assistant_core.app_configuration import AppConfiguration
from ai_assistant_core.extension.domain.base_extension_repository import (
    BaseExtensionRepository,
)
from ai_assistant_core.extension.infrastructure.whl_extension_repository import (
    WhlExtensionRepository,
)


class ExtensionModule(Module):
    def configure(self, binder: Binder):
        binder.bind(BaseExtensionRepository, to=WhlExtensionRepository)

    @singleton
    @provider
    def provide_whl_extension_repository(
        self, configuration: AppConfiguration
    ) -> WhlExtensionRepository:
        extensions_directory = configuration.extensions_directory
        return WhlExtensionRepository(extensions_directory=extensions_directory)
