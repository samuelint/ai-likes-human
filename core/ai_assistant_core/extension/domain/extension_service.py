from ai_assistant_core.extension.domain.base_extension_service import (
    BaseExtensionService,
)
from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto


class ExtensionService(BaseExtensionService):
    def __init__(self) -> None:
        pass

    def list_available(self) -> list[ExtensionInfoDto]:
        return []

    def list_installed(self) -> list[ExtensionInfoDto]:
        return []

    def install(self, extension: ExtensionInfoDto):
        pass

    def is_installed(self, extension: ExtensionInfoDto) -> bool:
        pass
