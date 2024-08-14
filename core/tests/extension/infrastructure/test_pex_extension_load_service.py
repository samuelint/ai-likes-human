from unittest.mock import patch
from decoy import Decoy, matchers
import psutil
import pytest

from ai_assistant_core.extension.domain.extension_load_state import (
    ExtensionLoadStateDto,
)
from ai_assistant_core.extension.infrastructure.in_memory_loaded_extension_repository import (
    InMemoryLoadedExtensionRepository,
)
from ai_assistant_core.extension.infrastructure.pex_extension_load_service import (
    PexExtensionLoadService,
)
from ai_assistant_core.extension.infrastructure.pex_process import PexProcess
from ai_assistant_core.extension.infrastructure.pex_process_factory import (
    PexProcessFactory,
)


@pytest.fixture
def loaded_extensions_repository(decoy: Decoy) -> InMemoryLoadedExtensionRepository:
    return decoy.mock(cls=InMemoryLoadedExtensionRepository)


@pytest.fixture
def pex_process_factory(decoy: Decoy) -> PexProcessFactory:
    return decoy.mock(cls=PexProcessFactory)


@pytest.fixture
def instance(
    loaded_extensions_repository: InMemoryLoadedExtensionRepository,
    pex_process_factory: PexProcessFactory,
) -> PexExtensionLoadService:
    return PexExtensionLoadService(
        loaded_extensions_repository=loaded_extensions_repository,
        pex_process_factory=pex_process_factory,
    )


class TestPexExtensionLoadService__load:

    def test_pex_process_is_created(
        self,
        decoy: Decoy,
        instance: PexExtensionLoadService,
        pex_process_factory: PexProcessFactory,
    ):
        mock_pex_process = decoy.mock(cls=PexProcess)
        decoy.when(pex_process_factory.create(extension_name="test")).then_return(
            mock_pex_process
        )

        result = instance.load(extension_name="test")

        assert result is mock_pex_process

    def test_pex_process_is_started(
        self,
        decoy: Decoy,
        instance: PexExtensionLoadService,
        pex_process_factory: PexProcessFactory,
    ):
        mock_pex_process = decoy.mock(cls=PexProcess)
        decoy.when(pex_process_factory.create(extension_name="test")).then_return(
            mock_pex_process
        )

        instance.load(extension_name="test")

        decoy.verify(mock_pex_process.start())

    def test_pex_process_is_registred(
        self,
        decoy: Decoy,
        instance: PexExtensionLoadService,
        loaded_extensions_repository: InMemoryLoadedExtensionRepository,
        pex_process_factory: PexProcessFactory,
    ):
        mock_pex_process = decoy.mock(cls=PexProcess)
        decoy.when(pex_process_factory.create(extension_name="test")).then_return(
            mock_pex_process
        )

        instance.load(extension_name="test")

        decoy.verify(
            loaded_extensions_repository.register(
                matchers.HasAttributes({"name": "test"})
            )
        )

    def test_pex_process_is_registred_with_ipc_port(
        self,
        decoy: Decoy,
        instance: PexExtensionLoadService,
        loaded_extensions_repository: InMemoryLoadedExtensionRepository,
        pex_process_factory: PexProcessFactory,
    ):
        mock_pex_process = decoy.mock(cls=PexProcess)
        decoy.when(mock_pex_process.ipc_port).then_return("654")
        decoy.when(pex_process_factory.create(extension_name="test")).then_return(
            mock_pex_process
        )

        instance.load(extension_name="test")

        decoy.verify(
            loaded_extensions_repository.register(
                matchers.HasAttributes({"name": "test", "ipc_port": "654"})
            )
        )

    def test_pex_process_is_registred_with_pid(
        self,
        decoy: Decoy,
        instance: PexExtensionLoadService,
        loaded_extensions_repository: InMemoryLoadedExtensionRepository,
        pex_process_factory: PexProcessFactory,
    ):
        mock_pex_process = decoy.mock(cls=PexProcess)
        decoy.when(mock_pex_process.pid).then_return("324")
        decoy.when(pex_process_factory.create(extension_name="test")).then_return(
            mock_pex_process
        )

        instance.load(extension_name="test")

        decoy.verify(
            loaded_extensions_repository.register(
                matchers.HasAttributes({"name": "test", "pid": "324"})
            )
        )


class TestPexExtensionLoadService__unload:

    def test_registered_extension_is_terminated(
        self,
        decoy: Decoy,
        instance: PexExtensionLoadService,
        loaded_extensions_repository: InMemoryLoadedExtensionRepository,
    ):
        registered_extension = decoy.mock(cls=ExtensionLoadStateDto)
        decoy.when(registered_extension.pid).then_return(234)
        decoy.when(loaded_extensions_repository.find_by_name(name="test")).then_return(
            registered_extension
        )

        mock_psutil_process = decoy.mock(cls=psutil.Process)

        def mock_process_impl(pid):
            if pid == 234:
                return mock_psutil_process
            return None

        with patch("psutil.Process") as mock_process:
            mock_process.side_effect = mock_process_impl

            instance.unload(extension_name="test")

        decoy.verify(mock_psutil_process.terminate())

    def test_registered_extension_un_registered(
        self,
        decoy: Decoy,
        instance: PexExtensionLoadService,
        loaded_extensions_repository: InMemoryLoadedExtensionRepository,
    ):
        registered_extension = decoy.mock(cls=ExtensionLoadStateDto)
        decoy.when(registered_extension.pid).then_return(234)
        decoy.when(loaded_extensions_repository.find_by_name(name="test")).then_return(
            registered_extension
        )

        with patch("psutil.Process"):
            instance.unload(extension_name="test")

        decoy.verify(loaded_extensions_repository.delete_by_name(name="test"))
