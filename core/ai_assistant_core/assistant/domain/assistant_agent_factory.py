from injector import inject
from .assistant_builder import AssistantBuilder
from ai_assistant_core.llm.domain.llm_factory import LLMFactory
from langchain_openai_api_bridge.core.agent_factory import AgentFactory, CreateAgentDto
from langgraph.graph.graph import CompiledGraph
from langchain_core.language_models import BaseChatModel


class AssistantAgentFactory(AgentFactory):

    @inject
    def __init__(self, llm_factory: LLMFactory) -> None:
        self.llm_factory = llm_factory

    def create_agent(self, llm: BaseChatModel, dto: CreateAgentDto) -> CompiledGraph:
        return AssistantBuilder(llm).build()

    def create_llm(self, dto: CreateAgentDto) -> CompiledGraph:
        return self.llm_factory.create_chat_model(
            vendor_model=dto.model,
            max_tokens=dto.max_tokens,
            temperature=dto.temperature,
        )
