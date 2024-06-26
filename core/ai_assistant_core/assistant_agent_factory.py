from ai_assistant_core.assistant_builder import AssistantBuilder
from ai_assistant_core.llm_provider_factory import LLMProviderFactory
from langchain_openai_api_bridge.core.agent_factory import AgentFactory, CreateAgentDto
from langgraph.graph.graph import CompiledGraph
from langchain_core.language_models import BaseChatModel


class AssistantAgentFactory(AgentFactory):

    def create_agent(self, llm: BaseChatModel, dto: CreateAgentDto) -> CompiledGraph:
        return AssistantBuilder(llm).build()

    def create_llm(self, dto: CreateAgentDto) -> CompiledGraph:
        return LLMProviderFactory().create(
            vendor_model=dto.model,
            max_tokens=dto.max_tokens,
            temperature=dto.temperature,
            api_key=dto.api_key,
        )
