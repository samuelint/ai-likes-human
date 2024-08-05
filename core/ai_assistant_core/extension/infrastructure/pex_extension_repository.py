import os
from typing import BinaryIO, Optional
from ai_assistant_core.extension.domain.base_extension_repository import (
    BaseExtensionRepository,
)
from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto
from ai_assistant_core.extension.infrastructure.localfile_extension_repository import (
    LocalFileExtensionRepository,
)


class PexExtensionRepository(BaseExtensionRepository):
    def __init__(self, extensions_directory: str) -> None:
        self.local_file_repository = LocalFileExtensionRepository(
            extensions_directory=extensions_directory,
            extension_file_format=".pex",
        )

    def list(self) -> list[ExtensionInfoDto]:
        whl_file_paths = self.local_file_repository.list()

        return [self._to_extension_dto(whl_file) for whl_file in whl_file_paths]

    def upload(self, filename: str, file: BinaryIO) -> ExtensionInfoDto:
        file_path = self.local_file_repository.upload(filename=filename, file=file)

        return self._to_extension_dto(file_path)

    def delete(self, name: str) -> ExtensionInfoDto:
        extension = self.get_by_name(name=name)
        self.local_file_repository.delete(extension.uri)

        return extension

    def find_by_name(self, name: str) -> Optional[ExtensionInfoDto]:
        available_extensions = self.list()
        for extension in available_extensions:
            if extension.name == name:
                return extension

    def _to_extension_dto(self, whl_file: str) -> ExtensionInfoDto:
        file_name = os.path.splitext(os.path.basename(whl_file))[0]

        return ExtensionInfoDto(
            name=file_name,
            version="unknown",
            author="unknown",
            uri=whl_file,
        )
