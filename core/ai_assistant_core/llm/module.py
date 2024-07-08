from injector import Binder, Module, multiprovider, provider, singleton
from ai_assistant_core.llm.domain.base_llm_factory import BaseLLMFactory
from ai_assistant_core.llm.domain.local_model_index_repository import (
    LocalLLMModelIndexRepository,
)
from ai_assistant_core.llm.infrastructure.anthropic_llm_factory import (
    AnthropicLLMFactory,
)
from ai_assistant_core.llm.infrastructure.llamacpp_llm_factory import LlamaCPPFactory
from ai_assistant_core.llm.infrastructure.llamacpp_proxy_factory import (
    LlamaCppProxyFactory,
)
from ai_assistant_core.llm.infrastructure.openai_llm_factory import OpenAILLMFactory
from ai_assistant_core.llm.infrastructure.sqlalchemy_local_model_index_repository import (
    SqlAlchemyLocalLLMModelIndexRepository,
)
from llama_cpp.server.app import LlamaProxy


class LLMModule(Module):
    def configure(self, binder: Binder):
        binder.bind(
            LocalLLMModelIndexRepository, to=SqlAlchemyLocalLLMModelIndexRepository
        )

    @provider
    @singleton
    def provide_llama_proxy(self, factory: LlamaCppProxyFactory) -> LlamaProxy:
        return factory.get_llama_proxy()

    @multiprovider
    def provide_llm_factories(
        self,
        llamacpp_factory: LlamaCPPFactory,
        openai_factory: OpenAILLMFactory,
        anthropic_factory: AnthropicLLMFactory,
    ) -> list[BaseLLMFactory]:
        return [llamacpp_factory, openai_factory, anthropic_factory]
