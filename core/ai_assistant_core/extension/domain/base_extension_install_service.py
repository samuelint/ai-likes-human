from abc import ABC, abstractmethod
from typing import Optional
from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto


class BaseExtensionInstallService(ABC):
    @abstractmethod
    def find_by_name(self, name: str) -> Optional[ExtensionInfoDto]:
        raise NotImplementedError()

    @abstractmethod
    def list_installed(self) -> list[ExtensionInfoDto]:
        raise NotImplementedError()

    @abstractmethod
    def install(self, extension: ExtensionInfoDto):
        raise NotImplementedError()

    @abstractmethod
    def uninstall_by_name(self, name: str) -> ExtensionInfoDto:
        raise NotImplementedError()

    @abstractmethod
    def uninstall(self, extension: ExtensionInfoDto) -> ExtensionInfoDto:
        raise NotImplementedError()

    @abstractmethod
    def is_installed(self, extension: ExtensionInfoDto) -> bool:
        raise NotImplementedError()
