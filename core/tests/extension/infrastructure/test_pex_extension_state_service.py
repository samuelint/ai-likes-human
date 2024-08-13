from decoy import Decoy
import pytest
from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto
from ai_assistant_core.extension.domain.extension_load_state import (
    ExtensionLoadStateDto,
)
from ai_assistant_core.extension.infrastructure.pex_extension_install_service import (
    PexExtensionInstallService,
)
from ai_assistant_core.extension.infrastructure.pex_extension_load_service import (
    PexExtensionLoadService,
)
from ai_assistant_core.extension.infrastructure.pex_extension_state_service import (
    PexExtensionStateService,
)


@pytest.fixture
def install_extension_service(decoy: Decoy) -> PexExtensionInstallService:
    return decoy.mock(cls=PexExtensionInstallService)


@pytest.fixture
def load_service(decoy: Decoy) -> PexExtensionLoadService:
    return decoy.mock(cls=PexExtensionLoadService)


@pytest.fixture
def instance(
    install_extension_service: PexExtensionInstallService,
    load_service: PexExtensionLoadService,
) -> PexExtensionStateService:
    return PexExtensionStateService(
        install_extension_service=install_extension_service,
        load_service=load_service,
    )


class TestPexExtensionStateService__find_by_name:
    def test_have_extension_name(
        self,
        decoy: Decoy,
        instance: PexExtensionStateService,
        install_extension_service: PexExtensionInstallService,
    ):
        some_info = ExtensionInfoDto(name="some", version="", author="", uri="")
        decoy.when(install_extension_service.find_by_name(name="some")).then_return(
            some_info
        )

        state = instance.find_by_name(name="some")

        assert state.name == "some"

    def test_have_extension_uri(
        self,
        decoy: Decoy,
        instance: PexExtensionStateService,
        install_extension_service: PexExtensionInstallService,
    ):
        some_info = ExtensionInfoDto(name="some", version="", author="", uri="uri")
        decoy.when(install_extension_service.find_by_name(name="some")).then_return(
            some_info
        )

        state = instance.find_by_name(name="some")

        assert state.uri == "uri"

    def test_have_extension_author(
        self,
        decoy: Decoy,
        instance: PexExtensionStateService,
        install_extension_service: PexExtensionInstallService,
    ):
        some_info = ExtensionInfoDto(name="some", version="", author="Me", uri="")
        decoy.when(install_extension_service.find_by_name(name="some")).then_return(
            some_info
        )

        state = instance.find_by_name(name="some")

        assert state.author == "Me"

    def test_have_extension_version(
        self,
        decoy: Decoy,
        instance: PexExtensionStateService,
        install_extension_service: PexExtensionInstallService,
    ):
        some_info = ExtensionInfoDto(name="some", version="1.0.1", author="", uri="")
        decoy.when(install_extension_service.find_by_name(name="some")).then_return(
            some_info
        )

        state = instance.find_by_name(name="some")

        assert state.version == "1.0.1"

    def test_loaded_extension_have_is_loaded_set_to_true(
        self,
        decoy: Decoy,
        instance: PexExtensionStateService,
        install_extension_service: PexExtensionInstallService,
        load_service: PexExtensionLoadService,
    ):
        some_info = ExtensionInfoDto(name="some", version="", author="", uri="")
        some_loaded_extension = ExtensionLoadStateDto(pid=0, ipc_port=0, name="some")
        decoy.when(install_extension_service.find_by_name(name="some")).then_return(
            some_info
        )
        decoy.when(
            load_service.find_loaded_extensions(extension_name="some")
        ).then_return(some_loaded_extension)

        state = instance.find_by_name(name="some")

        assert state.is_loaded is True

    def test_loaded_extension_have_pid(
        self,
        decoy: Decoy,
        instance: PexExtensionStateService,
        install_extension_service: PexExtensionInstallService,
        load_service: PexExtensionLoadService,
    ):
        some_info = ExtensionInfoDto(name="some", version="", author="", uri="")
        some_loaded_extension = ExtensionLoadStateDto(pid=111, ipc_port=0, name="some")
        decoy.when(install_extension_service.find_by_name(name="some")).then_return(
            some_info
        )
        decoy.when(
            load_service.find_loaded_extensions(extension_name="some")
        ).then_return(some_loaded_extension)

        state = instance.find_by_name(name="some")

        assert state.pid == 111

    def test_loaded_extension_have_ipc_port(
        self,
        decoy: Decoy,
        instance: PexExtensionStateService,
        install_extension_service: PexExtensionInstallService,
        load_service: PexExtensionLoadService,
    ):
        some_info = ExtensionInfoDto(name="some", version="", author="", uri="")
        some_loaded_extension = ExtensionLoadStateDto(pid=111, ipc_port=65, name="some")
        decoy.when(install_extension_service.find_by_name(name="some")).then_return(
            some_info
        )
        decoy.when(
            load_service.find_loaded_extensions(extension_name="some")
        ).then_return(some_loaded_extension)

        state = instance.find_by_name(name="some")

        assert state.ipc_port == 65


class TestPexExtensionStateService__list:

    def test_extensions_are_listed_with_name(
        self,
        decoy: Decoy,
        instance: PexExtensionStateService,
        install_extension_service: PexExtensionInstallService,
        load_service: PexExtensionLoadService,
    ):
        some_info1 = ExtensionInfoDto(name="some", version="", author="", uri="")
        decoy.when(
            load_service.find_loaded_extensions(extension_name="some")
        ).then_return(None)
        decoy.when(install_extension_service.list()).then_return([some_info1])

        result = instance.list()

        assert len(result) == 1
        assert result[0].name == "some"

    def test_extensions_are_listed_with_uri(
        self,
        decoy: Decoy,
        instance: PexExtensionStateService,
        install_extension_service: PexExtensionInstallService,
        load_service: PexExtensionLoadService,
    ):
        some_info1 = ExtensionInfoDto(name="some", version="", author="", uri="uri")
        decoy.when(
            load_service.find_loaded_extensions(extension_name="some")
        ).then_return(None)
        decoy.when(install_extension_service.list()).then_return([some_info1])

        result = instance.list()

        assert len(result) == 1
        assert result[0].uri == "uri"

    def test_loaded_extension_are_listed(
        self,
        decoy: Decoy,
        instance: PexExtensionStateService,
        install_extension_service: PexExtensionInstallService,
        load_service: PexExtensionLoadService,
    ):
        some_info1 = ExtensionInfoDto(name="some", version="", author="", uri="uri")
        some_loaded_extension1 = ExtensionLoadStateDto(pid=111, ipc_port=0, name="some")
        decoy.when(
            load_service.find_loaded_extensions(extension_name="some")
        ).then_return(some_loaded_extension1)
        decoy.when(install_extension_service.list()).then_return([some_info1])

        result = instance.list()

        assert len(result) == 1
        assert result[0].is_loaded is True
        assert result[0].pid == 111
