import pytest
from decoy import Decoy
from ai_assistant_core.llm.domain.api_key_service import ApiKeyService
from ai_assistant_core.llm.infrastructure.anthropic_llm_factory import (
    AnthropicLLMFactory,
)


class TestIsCompatible:

    @pytest.fixture
    def api_key_service(self, decoy: Decoy) -> ApiKeyService:
        service = decoy.mock(cls=ApiKeyService)
        decoy.when(service.get_anthropic_api_key()).then_return("")

        return service

    @pytest.fixture
    def instance(self, api_key_service: ApiKeyService) -> AnthropicLLMFactory:
        return AnthropicLLMFactory(api_key_service=api_key_service)

    def test_is_compatible_with_anthropic(self, instance: AnthropicLLMFactory):
        assert instance.is_compatible("anthropic")

    def test_is_not_compatible_with_not_anthropic(self, instance: AnthropicLLMFactory):
        assert instance.is_compatible("openai") is False


class TestCreate:

    @pytest.fixture
    def api_key_service(self, decoy: Decoy) -> ApiKeyService:
        service = decoy.mock(cls=ApiKeyService)
        decoy.when(service.get_anthropic_api_key()).then_return("")

        return service

    @pytest.fixture
    def instance(self, api_key_service: ApiKeyService) -> AnthropicLLMFactory:
        return AnthropicLLMFactory(api_key_service=api_key_service)

    def test_api_key_is_from_service(
        self,
        decoy: Decoy,
        instance: AnthropicLLMFactory,
        api_key_service: ApiKeyService,
    ):
        decoy.when(api_key_service.get_anthropic_api_key()).then_return("abcdefgh")

        result = instance.create(model="sonnet")

        assert result.anthropic_api_key._secret_value == "abcdefgh"

    def test_chat_model(self, instance: AnthropicLLMFactory):
        result = instance.create(model="sonnet")

        assert result.model == "sonnet"

    def test_default_temperature(self, instance: AnthropicLLMFactory):
        result = instance.create(model="sonnet")

        assert result.temperature == 0.7

    def test_chat_temperature(self, instance: AnthropicLLMFactory):
        result = instance.create(model="sonnet", temperature=0.2)

        assert result.temperature == 0.2

    def test_chat_max_tokens(self, instance: AnthropicLLMFactory):
        result = instance.create(model="sonnet", max_tokens=1024)

        assert result.max_tokens == 1024

    def test_chat_default_streaming_is_true(self, instance: AnthropicLLMFactory):
        result = instance.create(model="sonnet")

        assert result.streaming is True

    def test_chat_default_streaming(self, instance: AnthropicLLMFactory):
        result = instance.create(model="sonnet", streaming=False)

        assert result.streaming is False
