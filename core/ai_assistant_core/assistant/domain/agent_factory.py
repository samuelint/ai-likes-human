from abc import ABC, abstractmethod
from langchain_core.language_models import BaseChatModel
from langchain_core.runnables import Runnable


class BaseAgentFactory(ABC):
    def __init__(self):
        pass

    @abstractmethod
    def create(
        self,
        assistant_id: str,
        llm: BaseChatModel,
    ) -> Runnable:
        pass
