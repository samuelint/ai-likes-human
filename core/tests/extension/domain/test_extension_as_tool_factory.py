from base_assistant_extension import BaseExtension
from decoy import Decoy
import pytest
from ai_assistant_core.extension.domain.base_extension_repository import (
    BaseExtensionRepository,
)

from ai_assistant_core.extension.domain.extension_as_tool_factory import (
    ExtensionAsToolFactory,
)
from langchain_core.language_models import BaseChatModel

from ai_assistant_core.extension.infrastructure.pex_extension_install_service import (
    PexExtensionInstallService,
)


class TestExtensionAsToolFactory:

    @pytest.fixture
    def extension_repository(self, decoy: Decoy) -> BaseExtensionRepository:
        return decoy.mock(cls=BaseExtensionRepository)

    @pytest.fixture
    def extension_service(self, decoy: Decoy) -> PexExtensionInstallService:
        return decoy.mock(cls=PexExtensionInstallService)

    @pytest.fixture
    def llm(self, decoy: Decoy) -> BaseChatModel:
        return decoy.mock(cls=BaseChatModel)

    @pytest.fixture
    def instance(
        self,
        extension_repository: BaseExtensionRepository,
        extension_service: PexExtensionInstallService,
    ) -> ExtensionAsToolFactory:
        return ExtensionAsToolFactory(
            extension_repository=extension_repository,
            extension_service=extension_service,
        )

    def test_created_tool_name_has_no_spaces(
        self, instance: ExtensionAsToolFactory, llm: BaseChatModel
    ):

        class ExtensionFixture(BaseExtension):
            def name(self) -> str:
                return "my super extension"

            def description(self) -> str:
                return ""

            def create_runnable(self, **kwargs) -> None:
                return None

        tool = instance.create(extension=ExtensionFixture(), llm=llm)

        assert tool.name == "my_super_extension"

    def test_created_tool_description_is_the_same(
        self, instance: ExtensionAsToolFactory, llm: BaseChatModel
    ):

        class ExtensionFixture(BaseExtension):
            def name(self) -> str:
                return "my super extension"

            def description(self) -> str:
                return "some description"

            def create_runnable(self, **kwargs) -> None:
                return None

        tool = instance.create(extension=ExtensionFixture(), llm=llm)

        assert tool.description == "some description"
