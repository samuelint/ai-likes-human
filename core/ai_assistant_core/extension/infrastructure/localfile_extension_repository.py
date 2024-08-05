from typing import BinaryIO, Optional
import shutil
from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto
import os

from ai_assistant_core.extension.domain.invalid_file_format_error import (
    InvalidFileFormat,
)


class LocalFileExtensionRepository:
    def __init__(self, extensions_directory: str, extension_file_format: str) -> None:
        self.extensions_directory = extensions_directory
        self.extension_file_format = extension_file_format

    def list(self) -> list[str]:
        self._assert_extension_directory_exist()
        files: list[str] = []
        for entry in os.scandir(self.extensions_directory):
            if entry.is_file() and entry.path.endswith(self.extension_file_format):
                files.append(entry.path)

        return files

    def upload(self, filename: str, file: BinaryIO) -> str:
        if not filename.endswith(self.extension_file_format):
            raise InvalidFileFormat()

        self._assert_extension_directory_exist()

        file_path = os.path.join(self.extensions_directory, filename)
        with open(file_path, "wb") as buffer:
            shutil.copyfileobj(file, buffer)

        return file_path

    def delete(self, file: str) -> None:
        os.remove(file)

    def find_by_name(self, name: str) -> Optional[ExtensionInfoDto]:
        available_extensions = self.list_available()
        for extension in available_extensions:
            if extension.name == name:
                return extension

    def _assert_extension_directory_exist(self):
        if not os.path.exists(self.extensions_directory):
            os.makedirs(self.extensions_directory)
