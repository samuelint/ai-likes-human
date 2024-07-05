from injector import Binder, Module, multiprovider
from ai_assistant_core.llm.domain.base_llm_factory import BaseLLMFactory
from ai_assistant_core.llm.domain.local_model_index_repository import (
    LocalLLMModelIndexRepository,
)
from ai_assistant_core.llm.infrastructure.anthropic_llm_factory import (
    AnthropicLLMFactory,
)
from ai_assistant_core.llm.infrastructure.llamacpp_llm_factory import LlamaCPPFactory
from ai_assistant_core.llm.infrastructure.openai_llm_factory import OpenAILLMFactory
from ai_assistant_core.llm.infrastructure.sqlalchemy_local_model_index_repository import (
    SqlAlchemyLocalLLMModelIndexRepository,
)


class LLMModule(Module):
    def configure(self, binder: Binder):
        binder.bind(
            LocalLLMModelIndexRepository, to=SqlAlchemyLocalLLMModelIndexRepository
        )

    # @multiprovider
    # def provide_model_settings(self) -> dict[str, ModelSettings]:
    #     return {
    #         "llama3": ModelSettings(
    #             model="meta-llama/Meta-Llama-3-8B",
    #             model_alias="llama-7b",
    #             hf_model_repo_id="meta-llama/Meta-Llama-3-8B",
    #         ),
    #         "phi3": ModelSettings(
    #             model="microsoft/Phi-3-mini-4k-instruct",
    #             model_alias="llama-7b",
    #             hf_model_repo_id="microsoft/Phi-3-mini-4k-instruct",
    #         ),
    #     }

    @multiprovider
    def provide_llm_factories(
        self,
        llamacpp_factory: LlamaCPPFactory,
        openai_factory: OpenAILLMFactory,
        anthropic_factory: AnthropicLLMFactory,
    ) -> list[BaseLLMFactory]:
        return [llamacpp_factory, openai_factory, anthropic_factory]
