from abc import ABC, abstractmethod
from typing import Optional
from langchain_core.language_models import BaseChatModel
from langchain_core.tools import BaseTool
from langgraph.graph.graph import CompiledGraph


class BaseAgentFactory(ABC):
    def __init__(self):
        pass

    @abstractmethod
    def create(
        self, llm: BaseChatModel, tools: Optional[list[BaseTool]] = [], **kwargs
    ) -> CompiledGraph:
        pass
