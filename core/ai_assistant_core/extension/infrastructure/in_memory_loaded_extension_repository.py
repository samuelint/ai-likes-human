from typing import Optional
from injector import singleton
from ai_assistant_core.extension.domain.extension_load_state import (
    ExtensionLoadStateDto,
)


@singleton
class InMemoryLoadedExtensionRepository:
    _loaded_extensions: dict[str, ExtensionLoadStateDto]

    def __init__(self) -> None:
        self._loaded_extensions = {}

    def list(self) -> list[ExtensionLoadStateDto]:
        return list(self._loaded_extensions.values())

    def find_by_name(self, name: str) -> Optional[ExtensionLoadStateDto]:
        return self._loaded_extensions.get(name, None)

    def get_by_name(self, name: str) -> ExtensionLoadStateDto:
        result = self._loaded_extensions.get(name, None)

        if result is None:
            raise ValueError(f"Extension {name} not found")

        return result

    def register(self, extension_load_state: ExtensionLoadStateDto) -> None:
        if self.find_by_name(extension_load_state.name) is not None:
            raise ValueError(
                f"Extension {extension_load_state.name} already registered"
            )

        self._loaded_extensions[extension_load_state.name] = extension_load_state

    def delete_by_name(self, name: str) -> None:
        self._loaded_extensions.pop(name, None)
