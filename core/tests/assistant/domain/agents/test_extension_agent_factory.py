from decoy import Decoy, matchers
import pytest

from ai_assistant_core.assistant.domain.agents.extension_agent_factory import (
    ExtensionAgentFactory,
)
from ai_assistant_core.extension.domain.base_extension_repository import (
    BaseExtensionRepository,
)
from ai_assistant_core.extension.domain.extension_as_tool_factory import (
    ExtensionAsToolFactory,
)
from ai_assistant_core.extension.infrastructure.pex_extension_install_service import (
    PexExtensionInstallService,
)
from tests.extension.domain.__dto_factories__ import ExtensionInfoDtoPolyfactory


@pytest.fixture
def extension_repository(decoy: Decoy) -> BaseExtensionRepository:
    repo = decoy.mock(cls=BaseExtensionRepository)

    return repo


@pytest.fixture
def extension_service(decoy: Decoy) -> PexExtensionInstallService:
    repo = decoy.mock(cls=PexExtensionInstallService)

    return repo


@pytest.fixture
def extension_as_tool_factory(decoy: Decoy) -> ExtensionAsToolFactory:
    repo = decoy.mock(cls=ExtensionAsToolFactory)

    return repo


@pytest.fixture
def instance(
    extension_repository: BaseExtensionRepository,
    extension_service: PexExtensionInstallService,
    extension_as_tool_factory: ExtensionAsToolFactory,
) -> ExtensionAgentFactory:
    return ExtensionAgentFactory(
        extension_repository=extension_repository,
        extension_service=extension_service,
        extension_as_tool_factory=extension_as_tool_factory,
    )


class TestIsAssistantAnExtension:
    def test_assistant_id_correspond_to_extension_name_is_an_extension_agent(
        self,
        decoy: Decoy,
        instance: ExtensionAgentFactory,
        extension_repository: BaseExtensionRepository,
    ):
        some_extension_dto = ExtensionInfoDtoPolyfactory().build()
        decoy.when(extension_repository.find_by_name(name="joke")).then_return(
            some_extension_dto
        )

        result = instance.is_assistant_an_extension(assistant_id="joke")

        assert result is True

    def test_assistant_id_does_not_correspond_to_extension_name_is_not_an_extension_agent(
        self,
        decoy: Decoy,
        instance: ExtensionAgentFactory,
        extension_repository: BaseExtensionRepository,
    ):
        decoy.when(
            extension_repository.find_by_name(name=matchers.Anything())
        ).then_return(None)

        result = instance.is_assistant_an_extension(assistant_id="joke")

        assert result is False
