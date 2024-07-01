from injector import Binder, Module

from ai_assistant_core.llm.domain.base_llm_factory import BaseLLMFactory
from ai_assistant_core.llm.infrastructure.anthropic_llm_factory import (
    AnthropicLLMFactory,
)
from ai_assistant_core.llm.infrastructure.openai_llm_factory import OpenAILLMFactory


class LLMModule(Module):
    def configure(self, binder: Binder):
        binder.multibind(
            list[BaseLLMFactory], to=[OpenAILLMFactory, AnthropicLLMFactory]
        )
