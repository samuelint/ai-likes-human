import pytest
from ai_assistant_core.extension.domain.extension_load_state import (
    ExtensionLoadStateDto,
)
from ai_assistant_core.extension.infrastructure.in_memory_loaded_extension_repository import (
    InMemoryLoadedExtensionRepository,
)


class TestCrud:
    @pytest.fixture
    def instance(self) -> InMemoryLoadedExtensionRepository:
        return InMemoryLoadedExtensionRepository()

    def test_create_and_list(self, instance: InMemoryLoadedExtensionRepository):
        instance.register(ExtensionLoadStateDto(pid=1, name="A"))

        dtos = instance.list()

        assert any(dto.pid == 1 and dto.name == "A" for dto in dtos)

    def test_find_by_name(self, instance: InMemoryLoadedExtensionRepository):
        instance.register(ExtensionLoadStateDto(pid=2, name="A"))
        instance.register(ExtensionLoadStateDto(pid=3, name="B"))

        item = instance.find_by_name(name="A")
        assert item is not None

        item = instance.find_by_name(name="B")
        assert item is not None

        item = instance.find_by_name(name="C")
        assert item is None

    def test_same_name_extension_throws(
        self, instance: InMemoryLoadedExtensionRepository
    ):
        instance.register(ExtensionLoadStateDto(pid=2, name="A"))

        with pytest.raises(ValueError):
            instance.register(ExtensionLoadStateDto(pid=3, name="A"))

    def test_delete_by_name(self, instance: InMemoryLoadedExtensionRepository):
        instance.register(ExtensionLoadStateDto(pid=2, name="A"))
        instance.delete_by_name(name="A")

        dtos = instance.list()

        assert len(dtos) == 0
