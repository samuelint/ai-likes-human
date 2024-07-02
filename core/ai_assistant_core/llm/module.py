from injector import Module, multiprovider

from ai_assistant_core.llm.domain.base_llm_factory import BaseLLMFactory
from ai_assistant_core.llm.infrastructure.anthropic_llm_factory import (
    AnthropicLLMFactory,
)
from ai_assistant_core.llm.infrastructure.openai_llm_factory import OpenAILLMFactory


class LLMModule(Module):
    @multiprovider
    def provide_llm_factories(
        self, openai_factory: OpenAILLMFactory, anthropic_factory: AnthropicLLMFactory
    ) -> list[BaseLLMFactory]:
        return [openai_factory, anthropic_factory]
