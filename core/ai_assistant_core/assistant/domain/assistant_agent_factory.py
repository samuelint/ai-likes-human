from injector import inject
from langchain_core.language_models import BaseChatModel
from langchain_core.runnables import Runnable
from ai_assistant_core.assistant.domain.agents.default.agent_factory import (
    DefaultAgentFactory,
)
from ai_assistant_core.llm.domain.llm_factory import LLMFactory
from langchain_openai_api_bridge.core.agent_factory import AgentFactory, CreateAgentDto
from langgraph.graph.graph import CompiledGraph


class AssistantAgentFactory(AgentFactory):

    @inject
    def __init__(
        self,
        llm_factory: LLMFactory,
        default_agent_factory: DefaultAgentFactory,
    ) -> None:
        self.llm_factory = llm_factory
        self.default_agent_factory = default_agent_factory

    def create_agent(self, llm: BaseChatModel, dto: CreateAgentDto) -> Runnable:

        return self.default_agent_factory.create(llm=llm)

    def create_llm(self, dto: CreateAgentDto) -> CompiledGraph:
        return self.llm_factory.create_chat_model(
            vendor_model=dto.model,
            max_tokens=dto.max_tokens,
            temperature=dto.temperature,
        )
