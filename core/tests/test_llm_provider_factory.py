import pytest
from ai_assistant_core.llm.domain.llm_factory import LLMFactory
from langchain_openai import ChatOpenAI
from langchain_anthropic import ChatAnthropic
from unittest.mock import patch


def test_not_having_vendor_throws():
    with pytest.raises(ValueError) as e:
        LLMFactory().create_chat_model(vendor_model="gpt-4o")
    assert str(e.value) == "model must be in the format vendor:model"


class TestCreateOpenai:
    def test_is_openai_instance(self):
        llm = LLMFactory().create_chat_model("openai:gpt-4o")

        assert isinstance(llm, ChatOpenAI)

    @patch("ai_assistant_core.llm_provider_factory.ChatOpenAI", autospec=True)
    def test_default_temperature_is_0_7(self, mock_chat_openai):
        LLMFactory().create_chat_model("openai:gpt-4o")

        call_args = mock_chat_openai.call_args
        assert call_args[1]["temperature"] == 0.7

    @patch("ai_assistant_core.llm_provider_factory.ChatOpenAI", autospec=True)
    def test_temperature_arg_is_passed(self, mock_chat_openai):
        LLMFactory().create_chat_model("openai:gpt-4o", temperature=1)

        call_args = mock_chat_openai.call_args
        assert call_args[1]["temperature"] == 1

    @patch("ai_assistant_core.llm_provider_factory.ChatOpenAI", autospec=True)
    def test_max_tokens_arg_is_passed(self, mock_chat_openai):
        LLMFactory().create_chat_model(vendor_model="openai:gpt-4o", max_tokens=100)

        call_args = mock_chat_openai.call_args
        assert call_args[1]["max_tokens"] == 100

    @patch("ai_assistant_core.llm_provider_factory.ChatOpenAI", autospec=True)
    def test_streaming_is_enabled(self, mock_chat_openai):
        LLMFactory().create_chat_model("openai:gpt-4o")

        call_args = mock_chat_openai.call_args
        assert call_args[1]["streaming"] is True

    @patch("ai_assistant_core.llm_provider_factory.ChatOpenAI", autospec=True)
    def test_api_key(self, mock_chat_openai):
        LLMFactory().create_chat_model("openai:gpt-4o", api_key="key")

        call_args = mock_chat_openai.call_args
        assert call_args[1]["api_key"] == "key"

    @patch("ai_assistant_core.llm_provider_factory.ChatOpenAI", autospec=True)
    def test_api_key_default(self, mock_chat_openai):
        LLMFactory().create_chat_model("openai:gpt-4o")

        call_args = mock_chat_openai.call_args
        assert call_args[1]["api_key"] is None


class TestCreateAnthropic:
    def test_is_openai_instance(self):
        llm = LLMFactory().create_chat_model("anthropic:claude-3")

        assert isinstance(llm, ChatAnthropic)

    @patch("ai_assistant_core.llm_provider_factory.ChatAnthropic", autospec=True)
    def test_default_max_tokens_is_1024(self, mock_chat_anthropic):
        LLMFactory().create_chat_model("anthropic:claude3")

        call_args = mock_chat_anthropic.call_args
        assert call_args[1]["max_tokens"] == 1024

    @patch("ai_assistant_core.llm_provider_factory.ChatAnthropic", autospec=True)
    def test_default_temperature_is_0_7(self, mock_chat_anthropic):
        LLMFactory().create_chat_model("anthropic:claude3")

        call_args = mock_chat_anthropic.call_args
        assert call_args[1]["temperature"] == 0.7

    @patch("ai_assistant_core.llm_provider_factory.ChatAnthropic", autospec=True)
    def test_temperature_arg_is_passed(self, mock_chat_anthropic):
        LLMFactory().create_chat_model("anthropic:claude3", temperature=1)

        call_args = mock_chat_anthropic.call_args
        assert call_args[1]["temperature"] == 1

    @patch("ai_assistant_core.llm_provider_factory.ChatAnthropic", autospec=True)
    def test_max_tokens_arg_is_passed(self, mock_chat_anthropic):
        LLMFactory().create_chat_model("anthropic:claude3", max_tokens=100)

        call_args = mock_chat_anthropic.call_args
        assert call_args[1]["max_tokens"] == 100

    @patch("ai_assistant_core.llm_provider_factory.ChatAnthropic", autospec=True)
    def test_streaming_is_enabled(self, mock_chat_anthropic):
        LLMFactory().create_chat_model("anthropic:claude3")

        call_args = mock_chat_anthropic.call_args
        assert call_args[1]["streaming"] is True

    @patch("ai_assistant_core.llm_provider_factory.ChatAnthropic", autospec=True)
    def test_api_key(self, mock_chat_openai):
        LLMFactory().create_chat_model("anthropic:claude3", api_key="key")

        call_args = mock_chat_openai.call_args
        assert call_args[1]["api_key"] == "key"

    @patch("ai_assistant_core.llm_provider_factory.ChatAnthropic", autospec=True)
    def test_api_key_default(self, mock_chat_openai):
        LLMFactory().create_chat_model("anthropic:claude3")

        call_args = mock_chat_openai.call_args
        assert call_args[1]["api_key"] is None
