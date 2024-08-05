from injector import Module, provider, singleton

from ai_assistant_core.app_configuration import AppConfiguration
from ai_assistant_core.extension.domain.base_extension_repository import (
    BaseExtensionRepository,
)
from ai_assistant_core.extension.infrastructure.pex_extension_repository import (
    PexExtensionRepository,
)


class ExtensionModule(Module):

    @singleton
    @provider
    def provide_pex_extension_repository(
        self, configuration: AppConfiguration
    ) -> PexExtensionRepository:
        extensions_directory = configuration.extensions_directory
        return PexExtensionRepository(extensions_directory=extensions_directory)

    @singleton
    @provider
    def provide_base_extension_repository(
        self, pex_extension_repository: PexExtensionRepository
    ) -> BaseExtensionRepository:
        return pex_extension_repository
