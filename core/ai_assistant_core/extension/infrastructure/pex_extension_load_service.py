from typing import Optional
from injector import inject
import psutil

from ai_assistant_core.extension.domain.extension_load_state import (
    ExtensionLoadStateDto,
)
from ai_assistant_core.extension.infrastructure.in_memory_loaded_extension_repository import (
    InMemoryLoadedExtensionRepository,
)
from ai_assistant_core.extension.infrastructure.pex_extension_is_running_service import (
    PexExtensionIsRunningService,
)
from ai_assistant_core.extension.infrastructure.pex_process import PexProcess
from ai_assistant_core.extension.infrastructure.pex_process_factory import (
    PexProcessFactory,
)


@inject
class PexExtensionLoadService:
    def __init__(
        self,
        loaded_extensions_repository: InMemoryLoadedExtensionRepository,
        pex_process_factory: PexProcessFactory,
        is_running_service: PexExtensionIsRunningService,
    ) -> None:
        self.loaded_extensions_repository = loaded_extensions_repository
        self.pex_process_factory = pex_process_factory
        self.is_running_service = is_running_service

    def find_loaded_extensions(
        self, extension_name: str
    ) -> Optional[ExtensionLoadStateDto]:
        return self.loaded_extensions_repository.find_by_name(name=extension_name)

    def get_loaded_extensions(self, extension_name: str) -> ExtensionLoadStateDto:
        extension = self.find_loaded_extensions(extension_name=extension_name)

        if extension is None:
            raise ValueError(f"Extension {extension_name} not loaded")

        return extension

    def assert_loaded_extension(self, extension_name: str) -> ExtensionLoadStateDto:
        loaded_extension = self.find_loaded_extensions(extension_name=extension_name)
        if loaded_extension is None:
            self.load(extension_name=extension_name)

        extension = self.find_loaded_extensions(extension_name=extension_name)

        if extension is None:
            raise ValueError(f"Cannot load extension {extension_name} not loaded")

        return extension

    def load(
        self,
        extension_name: str,
    ) -> PexProcess:
        pex_process = self.pex_process_factory.create(
            extension_name=extension_name,
        )
        pex_process.start()

        self._register_pex(
            pid=pex_process.pid,
            ipc_port=pex_process.ipc_port,
            extension_name=extension_name,
        )

        self.is_running_service.wait_for_running(extension_name=extension_name)

        return pex_process

    def unload(self, extension_name: str) -> None:
        extension_state = self._find_registered_extension(extension_name=extension_name)
        if extension_state is None:
            return

        self.loaded_extensions_repository.delete_by_name(name=extension_name)

        extension_subprocess = psutil.Process(extension_state.pid)
        extension_subprocess.terminate()

    def _find_extension_process(self, extension_name: str) -> Optional[psutil.Process]:
        extension_state = self.loaded_extensions_repository.find_by_name(
            name=extension_name
        )

        return psutil.Process(extension_state.pid)

    def _find_registered_extension(self, extension_name: str) -> ExtensionLoadStateDto:
        return self.loaded_extensions_repository.find_by_name(name=extension_name)

    def _register_pex(
        self, pid: int, ipc_port: int, extension_name: str
    ) -> ExtensionLoadStateDto:
        state = ExtensionLoadStateDto(pid=pid, ipc_port=ipc_port, name=extension_name)
        self.loaded_extensions_repository.register(state)

        return state
