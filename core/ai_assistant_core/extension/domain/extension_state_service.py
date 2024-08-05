from typing import Optional

from injector import inject
from ai_assistant_core.extension.domain.extension_state_dto import ExtensionStateDto
from ai_assistant_core.extension.infrastructure.pex_extension_install_service import (
    PexExtensionInstallService,
)
from ai_assistant_core.extension.infrastructure.pex_extension_load_service import (
    PexExtensionLoadService,
)


@inject
class ExtensionStateService:
    def __init__(
        self,
        extension_service: PexExtensionInstallService,
        load_service: PexExtensionLoadService,
    ) -> None:
        self.extension_service = extension_service
        self.load_service = load_service

    def find_by_name(self, name: str) -> Optional[ExtensionStateDto]:
        extension_info = self.extension_service.find_by_name(name=name)
        if extension_info is None:
            return None

        extension_process = self.load_service.find_extension_process(
            extension_name=extension_info.name
        )

        return ExtensionStateDto(
            is_loaded=extension_process is not None,
            pid=extension_process.pid,
            **extension_info.to_dict(),
        )
