import os
import subprocess
import sys
from typing import Optional
import zipfile
from injector import inject
import psutil

from ai_assistant_core.extension.domain.extension_load_state import (
    ExtensionLoadStateDto,
)
from ai_assistant_core.extension.infrastructure.in_memory_loaded_extension_repository import (
    InMemoryLoadedExtensionRepository,
)
from ai_assistant_core.extension.infrastructure.pex_extension_repository import (
    PexExtensionRepository,
)


@inject
class PexExtensionLoadService:
    def __init__(
        self,
        loaded_extensions_repository: InMemoryLoadedExtensionRepository,
        extensions_repository: PexExtensionRepository,
    ) -> None:
        self.loaded_extensions_repository = loaded_extensions_repository
        self.extensions_repository = extensions_repository

    def find_extension_process(self, extension_name: str) -> Optional[psutil.Process]:
        extension_state = self.loaded_extensions_repository.find_by_name(
            name=extension_name
        )

        return psutil.Process(extension_state.pid)

    def load(self, extension_name: str) -> None:
        extension_info = self.extensions_repository.find_by_name(name=extension_name)

        if extension_info is None:
            raise ValueError(f"Extension {extension_name} not installed")

        pex_process = self._run_pex_file(extension_info.uri)
        self._register_pex(pid=pex_process.pid, extension_name=extension_name)

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

    def _register_pex(self, pid: int, extension_name: str) -> ExtensionLoadStateDto:
        state = ExtensionLoadStateDto(pid=pid, name=extension_name)
        self.loaded_extensions_repository.register(state)

        return state

    def _run_pex_file(self, pex_path, *args) -> subprocess.Popen:
        extract_path = os.path.join(os.path.dirname(sys.executable), "pex_extract")

        os.makedirs(extract_path, exist_ok=True)

        with zipfile.ZipFile(pex_path, "r") as zip_ref:
            zip_ref.extractall(extract_path)

        pex_executable = os.path.join(extract_path, ".bootstrap", "pex")
        command = [sys.executable, pex_executable] + list(args)

        pex_process = subprocess.Popen(
            command,
        )

        return pex_process
