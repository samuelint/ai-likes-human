from typing import BinaryIO, Optional
import shutil
from ai_assistant_core.extension.domain.base_extension_repository import (
    BaseExtensionRepository,
)
from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto
import os

from ai_assistant_core.extension.domain.invalid_file_format_error import (
    InvalidFileFormat,
)
from ai_assistant_core.extension.infrastructure.whl_extension_loader import (
    WhlExtensionLoader,
)
from ai_assistant_core.extension.infrastructure.whl_metadata_loader import (
    WhlMetadataLoader,
)


class WhlExtensionRepository(BaseExtensionRepository):
    def __init__(self, extensions_directory: str) -> None:
        self.extensions_directory = extensions_directory

    def list_available(self) -> list[ExtensionInfoDto]:
        whl_file_paths = self._list_whl_files_in_extension_directory()

        return [
            self._whl_file_to_extension_dto(whl_file) for whl_file in whl_file_paths
        ]

    def upload(self, filename: str, file: BinaryIO) -> ExtensionInfoDto:
        if not filename.endswith(".whl"):
            raise InvalidFileFormat()

        self._assert_extension_directory_exist()

        file_path = os.path.join(self.extensions_directory, filename)
        with open(file_path, "wb") as buffer:
            shutil.copyfileobj(file, buffer)

        return self._whl_file_to_extension_dto(file_path)

    def delete(self, name: str) -> ExtensionInfoDto:
        extension = self.get_by_name(name=name)
        os.remove(extension.uri)

        return extension

    def find_by_name(self, name: str) -> Optional[ExtensionInfoDto]:
        available_extensions = self.list_available()
        for extension in available_extensions:
            if extension.name == name:
                return extension

    def list_installed(self) -> list[ExtensionInfoDto]:
        available_extensions = self.list_available()

        installed_extensions: list[ExtensionInfoDto] = []

        for extension in available_extensions:
            if self.is_installed(extension):
                installed_extensions.append(extension)

        return installed_extensions

    def install(self, extension: ExtensionInfoDto):
        loader = WhlExtensionLoader(wheel_path=extension.uri)
        loader.install()

    def is_installed(self, extension: ExtensionInfoDto) -> bool:
        loader = WhlExtensionLoader(wheel_path=extension.uri)
        return loader.is_installed()

    def _assert_extension_directory_exist(self):
        if not os.path.exists(self.extensions_directory):
            os.makedirs(self.extensions_directory)

    def _whl_file_to_extension_dto(self, whl_file: str) -> ExtensionInfoDto:
        metadata = WhlMetadataLoader(wheel_path=whl_file).read_metadata_as_set()
        return ExtensionInfoDto(
            name=metadata.get("name", "unknown"),
            version=metadata.get("version", "unknown"),
            author=metadata.get("author", "unknown"),
            uri=whl_file,
        )

    def _list_whl_files_in_extension_directory(self) -> list[str]:
        whl_files: list[str] = []
        for entry in os.scandir(self.extensions_directory):
            if entry.is_file() and entry.path.endswith(".whl"):
                whl_files.append(entry.path)

        return whl_files
