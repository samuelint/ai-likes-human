from injector import Module, provider, singleton

from ai_assistant_core.app_configuration import AppConfiguration
from ai_assistant_core.extension.infrastructure.whl_extension_service import (
    WhlExtensionService,
)


class ExtensionModule(Module):
    @singleton
    @provider
    def provide_whl_extension_service(
        self, configuration: AppConfiguration
    ) -> WhlExtensionService:
        extensions_directory = configuration.extensions_directory
        return WhlExtensionService(extensions_directory=extensions_directory)
