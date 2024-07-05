from unittest.mock import patch
from decoy import Decoy, matchers
import pytest
from llama_cpp.server.settings import ModelSettings
from ai_assistant_core.llm.domain.local_model_dto import FileLLMModelIndex
from ai_assistant_core.llm.domain.local_model_service import LocalLLMModelService
from ai_assistant_core.llm.infrastructure.llamacpp_llm_factory import LlamaCPPFactory
from ai_assistant_core.llm.infrastructure.llamacpp_model_settings_factory import (
    LlamaCppModelSettingsFactory,
)
from tests.llm.domain.__dto_factories__ import (
    LLMModelIndexFactory,
)


class TestIsCompatible:

    @pytest.fixture
    def instance(self) -> LlamaCPPFactory:
        return LlamaCPPFactory()

    def test_is_compatible_with_local(self, instance: LlamaCPPFactory):
        assert instance.is_compatible("local")

    def test_is_not_compatible_with_not_openai(self, instance: LlamaCPPFactory):
        assert instance.is_compatible("anthropic") is False


class TestCreate:
    @pytest.fixture
    def phi3_model_index(self) -> FileLLMModelIndex:
        return LLMModelIndexFactory.build(name="phi3", local_path="./q4.gguf")

    @pytest.fixture
    def local_model_service(
        self, decoy: Decoy, phi3_model_index: FileLLMModelIndex
    ) -> LocalLLMModelService:
        service = decoy.mock(cls=LocalLLMModelService)
        decoy.when(service.list()).then_return([phi3_model_index])

        return service

    @pytest.fixture
    def model_settings_factory(self, decoy: Decoy) -> LlamaCppModelSettingsFactory:
        service = decoy.mock(cls=LlamaCppModelSettingsFactory)
        decoy.when(
            service.create(local_path=matchers.Anything(), model_alias="phi3")
        ).then_return(ModelSettings(model="./q4.gguf", model_alias="phi3", n_ctx=800))

        return service

    @pytest.fixture
    def instance(
        self,
        local_model_service: LocalLLMModelService,
        model_settings_factory: LlamaCppModelSettingsFactory,
    ) -> LlamaCPPFactory:
        return LlamaCPPFactory(
            local_model_service=local_model_service,
            model_settings_factory=model_settings_factory,
        )

    @patch(
        "ai_assistant_core.llm.infrastructure.llamacpp_llm_factory.create_app",
        autospec=True,
    )
    def test_created_with_model_settings(
        self, mock_create_app, instance: LlamaCPPFactory
    ):
        instance.create(model="phi3")

        mock_create_app.assert_called_once()
        call_args = mock_create_app.call_args
        _, kwargs = call_args
        assert kwargs["model_settings"][0].model == "./q4.gguf"
        assert kwargs["model_settings"][0].n_ctx == 800
        assert kwargs["model_settings"][0].model_alias == "phi3"

    def test_not_supported_model_throws(self, instance: LlamaCPPFactory):
        with pytest.raises(ValueError):
            instance.create(model="some")

    @patch(
        "ai_assistant_core.llm.infrastructure.llamacpp_llm_factory.create_app",
        autospec=True,
    )
    def test_chat_model(self, mock_create_app, instance: LlamaCPPFactory):
        result = instance.create(model="phi3")

        assert result.model_name == "phi3"

    @patch(
        "ai_assistant_core.llm.infrastructure.llamacpp_llm_factory.create_app",
        autospec=True,
    )
    def test_default_temperature(self, mock_create_app, instance: LlamaCPPFactory):
        result = instance.create(model="phi3")

        assert result.temperature == 0.7

    @patch(
        "ai_assistant_core.llm.infrastructure.llamacpp_llm_factory.create_app",
        autospec=True,
    )
    def test_chat_temperature(self, mock_create_app, instance: LlamaCPPFactory):
        result = instance.create(model="phi3", temperature=0.2)

        assert result.temperature == 0.2

    @patch(
        "ai_assistant_core.llm.infrastructure.llamacpp_llm_factory.create_app",
        autospec=True,
    )
    def test_chat_max_tokens(self, mock_create_app, instance: LlamaCPPFactory):
        result = instance.create(model="phi3", max_tokens=1024)

        assert result.max_tokens == 1024

    @patch(
        "ai_assistant_core.llm.infrastructure.llamacpp_llm_factory.create_app",
        autospec=True,
    )
    def test_chat_default_streaming_is_true(
        self, mock_create_app, instance: LlamaCPPFactory
    ):
        result = instance.create(model="phi3")

        assert result.streaming is True

    @patch(
        "ai_assistant_core.llm.infrastructure.llamacpp_llm_factory.create_app",
        autospec=True,
    )
    def test_chat_default_streaming(self, mock_create_app, instance: LlamaCPPFactory):
        result = instance.create(model="phi3", streaming=False)

        assert result.streaming is False
