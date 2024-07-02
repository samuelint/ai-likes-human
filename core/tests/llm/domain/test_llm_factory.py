from typing import Optional, cast
import pytest
from langchain_core.language_models import BaseChatModel
from ai_assistant_core.llm.domain.base_llm_factory import BaseLLMFactory
from ai_assistant_core.llm.domain.llm_factory import LLMFactory


class SomeChatModelA:
    def __init__(
        self, model: str, max_tokens: Optional[int] = None, temperature: float = None
    ):
        super().__init__()
        self.model = model
        self.max_tokens = max_tokens
        self.temperature = temperature

    def _llm_type(self):
        return "a"

    def _generate(self, *args, **kwargs):
        pass


class LLMFactoryA(BaseLLMFactory):
    def is_compatible(self, vendor: str) -> bool:
        return vendor.lower() == "a"

    def create(
        self,
        model: str,
        max_tokens: Optional[int] = None,
        temperature: Optional[float] = 0.7,
    ) -> BaseChatModel:
        return SomeChatModelA(
            model=model, max_tokens=max_tokens, temperature=temperature
        )


class TestCreateChatModel:

    @pytest.fixture
    def factory_a(self) -> LLMFactoryA:
        return LLMFactoryA()

    @pytest.fixture
    def instance(self, factory_a) -> LLMFactory:
        return LLMFactory(factories=[factory_a])

    def test_create_chat_model(self, instance: LLMFactory):
        result = instance.create_chat_model(vendor_model="a:some")

        assert isinstance(result, SomeChatModelA)

    def test_llm_have_model(self, instance: LLMFactory):
        result = instance.create_chat_model(vendor_model="a:some")
        result = cast(SomeChatModelA, result)

        assert result.model == "some"

    def test_llm_have_temperature(self, instance: LLMFactory):
        result = instance.create_chat_model(vendor_model="a:some", temperature=0.9)
        result = cast(SomeChatModelA, result)

        assert result.temperature == 0.9

    def test_llm_have_max_tokens(self, instance: LLMFactory):
        result = instance.create_chat_model(vendor_model="a:some", max_tokens=11234)
        result = cast(SomeChatModelA, result)

        assert result.max_tokens == 11234

    def test_unsupported_vendor_throws(self, instance: LLMFactory):
        with pytest.raises(ValueError, match="Unknown LLM vendor: zzz"):
            instance.create_chat_model(vendor_model="zzz:some")
