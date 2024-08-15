from decoy import Decoy
from langchain_openai import ChatOpenAI
import pytest
from ai_assistant_core.extension.infrastructure.pex_extension_api_factory import (
    PexExtensionApiFactory,
)
from ai_assistant_core.extension.infrastructure.pex_extension_inference_factory import (
    PexExtensionInferenceFactory,
)
from ai_assistant_core.extension.infrastructure.pex_extension_load_service import (
    PexExtensionLoadService,
)
from ai_assistant_core.extension.infrastructure.pex_extension_metadata_api import (
    PexExtensionApi,
)


class TestPexExtensionInferenceFactory:

    @pytest.fixture
    def extension_api(self, decoy: Decoy) -> PexExtensionApi:
        return decoy.mock(cls=PexExtensionApi)

    @pytest.fixture
    def load_service(self, decoy: Decoy) -> PexExtensionLoadService:
        return decoy.mock(cls=PexExtensionLoadService)

    @pytest.fixture
    def extension_api_factory(
        self,
        decoy: Decoy,
        extension_api: PexExtensionApi,
    ) -> PexExtensionApiFactory:
        extension_api_mock = decoy.mock(cls=PexExtensionApiFactory)

        decoy.when(extension_api.get_metadata()).then_return(
            {"name": "Some Name", "description": "Some Description"}
        )
        decoy.when(
            extension_api_mock.create_from_extension_name(extension_name="some")
        ).then_return(extension_api)

        return extension_api_mock

    @pytest.fixture
    def instance(
        self,
        extension_api_factory: PexExtensionApiFactory,
        load_service: PexExtensionLoadService,
    ) -> PexExtensionInferenceFactory:
        return PexExtensionInferenceFactory(
            extension_api_factory=extension_api_factory, load_service=load_service
        )

    def test_extension_name_is_from_metadata(
        self,
        instance: PexExtensionInferenceFactory,
    ):
        result = instance.create(
            extension_name="some",
            extension_llm_model="openai:gpt-4o-mini",
        )

        assert result.name == "Some Name"

    def test_extension_description_is_from_metadata(
        self,
        instance: PexExtensionInferenceFactory,
    ):
        result = instance.create(
            extension_name="some",
            extension_llm_model="openai:gpt-4o-mini",
        )

        assert result.description == "Some Description"

    def test_extension_runnable_is_a_proxy_to_self_runnable(
        self,
        decoy: Decoy,
        extension_api: PexExtensionApi,
        instance: PexExtensionInferenceFactory,
    ):
        runnable_mock = decoy.mock(cls=ChatOpenAI)
        decoy.when(
            extension_api.get_proxy_openai_chat_client(model="openai:gpt-4o-mini")
        ).then_return(runnable_mock)

        result = instance.create(
            extension_name="some",
            extension_llm_model="openai:gpt-4o-mini",
        )

        assert result.runnable is runnable_mock

    def test_extension_is_loaded(
        self,
        decoy: Decoy,
        instance: PexExtensionInferenceFactory,
        load_service: PexExtensionLoadService,
    ):
        instance.create(
            extension_name="some",
            extension_llm_model="openai:gpt-4o-mini",
        )

        decoy.verify(load_service.assert_loaded_extension(extension_name="some"))
