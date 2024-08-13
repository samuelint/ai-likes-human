from typing import Optional

from injector import inject
from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto
from ai_assistant_core.extension.domain.extension_state_dto import ExtensionStateDto
from ai_assistant_core.extension.infrastructure.pex_extension_install_service import (
    PexExtensionInstallService,
)
from ai_assistant_core.extension.infrastructure.pex_extension_load_service import (
    PexExtensionLoadService,
)


@inject
class PexExtensionStateService:
    def __init__(
        self,
        install_extension_service: PexExtensionInstallService,
        load_service: PexExtensionLoadService,
    ) -> None:
        self.extension_service = install_extension_service
        self.load_service = load_service

    def list(self) -> list[ExtensionStateDto]:
        extension_info = self.extension_service.list()

        return [
            self._to_extension_state(extension_info=info) for info in extension_info
        ]

    def find_by_name(self, name: str) -> Optional[ExtensionStateDto]:
        extension_info = self.extension_service.find_by_name(name=name)
        if extension_info is None:
            return None

        return self._to_extension_state(extension_info=extension_info)

    def _to_extension_state(
        self, extension_info: ExtensionInfoDto
    ) -> ExtensionStateDto:
        loaded_extension = self.load_service.find_loaded_extensions(
            extension_name=extension_info.name
        )

        is_loaded = loaded_extension is not None
        ipc_port = loaded_extension.ipc_port if is_loaded else None
        pid = loaded_extension.pid if is_loaded else None

        return ExtensionStateDto(
            is_loaded=is_loaded,
            ipc_port=ipc_port,
            pid=pid,
            **extension_info.to_dict(),
        )
