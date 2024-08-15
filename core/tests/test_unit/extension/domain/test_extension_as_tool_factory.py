from decoy import Decoy
import pytest
from ai_assistant_core.extension.domain.base_extension_inference_factory import (
    BaseExtensionInferenceFactory,
)
from ai_assistant_core.extension.domain.base_extension_repository import (
    BaseExtensionRepository,
)

from ai_assistant_core.extension.domain.extension_as_tool_factory import (
    ExtensionAsToolFactory,
)
from ai_assistant_core.extension.domain.inferable_extension import InferableExtension


class TestExtensionAsToolFactory:

    @pytest.fixture
    def extension_repository(self, decoy: Decoy) -> BaseExtensionRepository:
        return decoy.mock(cls=BaseExtensionRepository)

    @pytest.fixture
    def extension_inference_factory(
        self, decoy: Decoy
    ) -> BaseExtensionInferenceFactory:
        return decoy.mock(cls=BaseExtensionInferenceFactory)

    @pytest.fixture
    def instance(
        self,
        extension_repository: BaseExtensionRepository,
        extension_inference_factory: BaseExtensionInferenceFactory,
    ) -> ExtensionAsToolFactory:
        return ExtensionAsToolFactory(
            extension_repository=extension_repository,
            extension_inference_factory=extension_inference_factory,
        )

    def test_created_tool_name_has_no_spaces(
        self,
        decoy: Decoy,
        instance: ExtensionAsToolFactory,
        extension_inference_factory: BaseExtensionInferenceFactory,
    ):
        inferable_extension = InferableExtension(
            name="my super extension", description="some description", runnable=None
        )
        decoy.when(
            extension_inference_factory.create(
                extension_name="some-extension", extension_llm_model="gpt4"
            )
        ).then_return(inferable_extension)

        tool = instance.create(
            extension_name="some-extension", extension_llm_model="gpt4"
        )

        assert tool.name == "my_super_extension"

    def test_created_tool_description_is_the_same(
        self,
        decoy: Decoy,
        instance: ExtensionAsToolFactory,
        extension_inference_factory: BaseExtensionInferenceFactory,
    ):
        inferable_extension = InferableExtension(
            name="my super extension", description="some description", runnable=None
        )
        decoy.when(
            extension_inference_factory.create(
                extension_name="some-extension", extension_llm_model="gpt4"
            )
        ).then_return(inferable_extension)

        tool = instance.create(
            extension_name="some-extension", extension_llm_model="gpt4"
        )

        assert tool.description == "some description"
