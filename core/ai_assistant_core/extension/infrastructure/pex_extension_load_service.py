from typing import Optional
from injector import inject
import psutil

from ai_assistant_core.extension.domain.extension_load_state import (
    ExtensionLoadStateDto,
)
from ai_assistant_core.extension.infrastructure.extension_ipc_port_service import (
    ExtensionIpcPortService,
)
from ai_assistant_core.extension.infrastructure.in_memory_loaded_extension_repository import (
    InMemoryLoadedExtensionRepository,
)
from ai_assistant_core.extension.infrastructure.pex_extension_inference_url_factory import (
    PexExtensionInferenceUrlFactory,
)
from ai_assistant_core.extension.infrastructure.pex_extension_repository import (
    PexExtensionRepository,
)
from ai_assistant_core.extension.infrastructure.pex_process import PexProcess


@inject
class PexExtensionLoadService:
    def __init__(
        self,
        loaded_extensions_repository: InMemoryLoadedExtensionRepository,
        extensions_repository: PexExtensionRepository,
        inference_url_factory: PexExtensionInferenceUrlFactory,
    ) -> None:
        self.loaded_extensions_repository = loaded_extensions_repository
        self.extensions_repository = extensions_repository
        self.inference_url_factory = inference_url_factory

    def find_loaded_extensions(
        self, extension_name: str
    ) -> Optional[ExtensionLoadStateDto]:
        return self.loaded_extensions_repository.find_by_name(name=extension_name)

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
        ipc_port: Optional[int] = None,
        inference_url: Optional[str] = None,
    ) -> PexProcess:
        ipc_port = ipc_port or ExtensionIpcPortService.find_next_available_port()
        inference_url = (
            inference_url or self.inference_url_factory.get_self_inference_url()
        )
        extension_info = self.extensions_repository.find_by_name(name=extension_name)

        if extension_info is None:
            raise ValueError(f"Extension {extension_name} not installed")

        pex_process = PexProcess(
            pex_path=extension_info.uri,
            ipc_port=ipc_port,
            inference_url=inference_url,
        )
        pex_process.start()

        self._register_pex(
            pid=pex_process.pid,
            ipc_port=ipc_port,
            extension_name=extension_name,
        )

        return pex_process

    def unload(self, extension_name: str) -> None:
        extension_state = self.loaded_extensions_repository.find_by_name(
            name=extension_name
        )
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

    def _register_pex(
        self, pid: int, ipc_port: int, extension_name: str
    ) -> ExtensionLoadStateDto:
        state = ExtensionLoadStateDto(pid=pid, ipc_port=ipc_port, name=extension_name)
        self.loaded_extensions_repository.register(state)

        return state
