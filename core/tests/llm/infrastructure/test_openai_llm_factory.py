import pytest
from decoy import Decoy
from ai_assistant_core.llm.domain.api_key_service import ApiKeyService
from ai_assistant_core.llm.infrastructure.openai_llm_factory import OpenAILLMFactory


class TestIsCompatible:

    @pytest.fixture
    def instance(self) -> OpenAILLMFactory:
        return OpenAILLMFactory()

    def test_is_compatible_with_openai(self, instance: OpenAILLMFactory):
        assert instance.is_compatible("openai")

    def test_is_not_compatible_with_not_openai(self, instance: OpenAILLMFactory):
        assert instance.is_compatible("anthropic") is False


class TestCreate:

    @pytest.fixture
    def api_key_service(self, decoy: Decoy) -> ApiKeyService:
        service = decoy.mock(cls=ApiKeyService)
        decoy.when(service.get_openai_api_key()).then_return("")

        return service

    @pytest.fixture
    def instance(self, api_key_service: ApiKeyService) -> OpenAILLMFactory:
        return OpenAILLMFactory(api_key_service=api_key_service)

    def test_api_key_is_from_service(
        self, decoy: Decoy, instance: OpenAILLMFactory, api_key_service: ApiKeyService
    ):
        decoy.when(api_key_service.get_openai_api_key()).then_return("abcdefgh")

        result = instance.create(model="gpt4")

        assert result.openai_api_key._secret_value == "abcdefgh"

    def test_chat_model(self, instance: OpenAILLMFactory):
        result = instance.create(model="gpt4")

        assert result.model_name == "gpt4"

    def test_default_temperature(self, instance: OpenAILLMFactory):
        result = instance.create(model="gpt4")

        assert result.temperature == 0.7

    def test_chat_temperature(self, instance: OpenAILLMFactory):
        result = instance.create(model="gpt4", temperature=0.2)

        assert result.temperature == 0.2

    def test_chat_max_tokens(self, instance: OpenAILLMFactory):
        result = instance.create(model="gpt4", max_tokens=1024)

        assert result.max_tokens == 1024

    def test_chat_default_streaming_is_true(self, instance: OpenAILLMFactory):
        result = instance.create(model="gpt4")

        assert result.streaming is True

    def test_chat_default_streaming(self, instance: OpenAILLMFactory):
        result = instance.create(model="gpt4", streaming=False)

        assert result.streaming is False
