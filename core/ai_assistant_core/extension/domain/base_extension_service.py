from abc import ABC, abstractmethod
from typing import BinaryIO, Optional
from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto


class BaseExtensionService(ABC):
    @abstractmethod
    def list_available(self) -> list[ExtensionInfoDto]:
        raise NotImplementedError()

    @abstractmethod
    def upload(self, file: BinaryIO) -> ExtensionInfoDto:
        raise NotImplementedError()

    @abstractmethod
    def delete(self, name: str) -> ExtensionInfoDto:
        raise NotImplementedError()

    @abstractmethod
    def find_by_name(self, name: str) -> Optional[ExtensionInfoDto]:
        raise NotImplementedError()

    def get_by_name(self, name: str) -> Optional[ExtensionInfoDto]:
        extension = self.find_by_name(name)
        if extension is None:
            raise Exception(f"Extension {name} not found")
        return extension

    @abstractmethod
    def list_installed(self) -> list[ExtensionInfoDto]:
        raise NotImplementedError()

    @abstractmethod
    def install(self, extension: ExtensionInfoDto):
        raise NotImplementedError()

    @abstractmethod
    def is_installed(self, extension: ExtensionInfoDto) -> bool:
        raise NotImplementedError()
