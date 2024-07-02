from typing import Optional
from injector import inject
from .agent_builder import AgentBuilder
from ai_assistant_core.llm.domain.llm_factory import LLMFactory
from langchain_openai_api_bridge.core.agent_factory import AgentFactory, CreateAgentDto
from langgraph.graph.graph import CompiledGraph
from langchain_core.language_models import BaseChatModel
from langchain_core.tools import BaseTool


class AssistantAgentFactory(AgentFactory):

    @inject
    def __init__(
        self, llm_factory: LLMFactory, tools: Optional[list[BaseTool]] = None
    ) -> None:
        self.llm_factory = llm_factory
        self.tools = tools

    def create_agent(self, llm: BaseChatModel, dto: CreateAgentDto) -> CompiledGraph:
        return AgentBuilder(llm=llm, tools=self.tools).build()

    def create_llm(self, dto: CreateAgentDto) -> CompiledGraph:
        return self.llm_factory.create_chat_model(
            vendor_model=dto.model,
            max_tokens=dto.max_tokens,
            temperature=dto.temperature,
        )
