from injector import inject
from langchain_core.language_models import BaseChatModel
from langchain_core.runnables import Runnable
from ai_assistant_core.assistant.domain.agents.default_agent_factory import (
    DefaultAgentFactory,
)
from ai_assistant_core.assistant.domain.agents.extension_agent_factory import (
    ExtensionAgentFactory,
)
from ai_assistant_core.llm.domain.llm_factory import LLMFactory
from langchain_openai_api_bridge.core.agent_factory import AgentFactory, CreateAgentDto
from langgraph.graph.graph import CompiledGraph


@inject
class AssistantAgentFactory(AgentFactory):

    def __init__(
        self,
        llm_factory: LLMFactory,
        default_agent_factory: DefaultAgentFactory,
        extension_agent_factory: ExtensionAgentFactory,
    ) -> None:
        self.llm_factory = llm_factory
        self.default_agent_factory = default_agent_factory
        self.extension_agent_factory = extension_agent_factory

    def create_agent(self, llm: BaseChatModel, dto: CreateAgentDto) -> Runnable:
        factory = self.default_agent_factory.create

        if self.extension_agent_factory.is_assistant_an_extension(
            assistant_id=dto.assistant_id
        ):
            factory = self.extension_agent_factory.create

        return factory(
            assistant_id=dto.assistant_id,
            llm=llm,
        )

    def create_llm(self, dto: CreateAgentDto) -> CompiledGraph:
        return self.llm_factory.create_chat_model(
            vendor_model=dto.model,
            max_tokens=dto.max_tokens,
            temperature=dto.temperature,
        )
