from injector import Binder, Module, singleton
from langchain_openai_api_bridge.assistant import (
    ThreadRepository,
    MessageRepository,
    RunRepository,
    InMemoryThreadRepository,
    InMemoryMessageRepository,
    InMemoryRunRepository,
)
from langchain_openai_api_bridge.core import AgentFactory
from .domain.assistant_agent_factory import AssistantAgentFactory


class AssistantModule(Module):
    def configure(self, binder: Binder):
        binder.bind(ThreadRepository, to=InMemoryThreadRepository, scope=singleton)
        binder.bind(MessageRepository, to=InMemoryMessageRepository, scope=singleton)
        binder.bind(RunRepository, to=InMemoryRunRepository, scope=singleton)
        binder.bind(AgentFactory, to=AssistantAgentFactory)
