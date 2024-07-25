from abc import ABC, abstractmethod
from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto


class BaseExtensionService(ABC):
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
