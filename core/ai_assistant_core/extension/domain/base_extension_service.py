from abc import ABC, abstractmethod
from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto
from base_assistant_extension.base_extension import (
    BaseExtension,
)


class BaseExtensionService(ABC):
    @abstractmethod
    def load(self, extension: ExtensionInfoDto) -> BaseExtension:
        raise NotImplementedError()

    @abstractmethod
    def list_installed(self) -> list[ExtensionInfoDto]:
        raise NotImplementedError()

    @abstractmethod
    def install(self, extension: ExtensionInfoDto):
        raise NotImplementedError()

    @abstractmethod
    def uninstall(self, extension: ExtensionInfoDto):
        raise NotImplementedError()

    @abstractmethod
    def is_installed(self, extension: ExtensionInfoDto) -> bool:
        raise NotImplementedError()
