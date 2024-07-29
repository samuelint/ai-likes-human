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
from langchain_openai_api_bridge.core.create_agent_dto import CreateAgentDto
from langchain_openai_api_bridge.core import BaseAgentFactory
from langchain_openai_api_bridge.assistant import (
    ThreadRepository,
)


@inject
class AssistantAgentFactory(BaseAgentFactory):

    def __init__(
        self,
        llm_factory: LLMFactory,
        default_agent_factory: DefaultAgentFactory,
        extension_agent_factory: ExtensionAgentFactory,
        thread_repository: ThreadRepository,
    ) -> None:
        self.llm_factory = llm_factory
        self.default_agent_factory = default_agent_factory
        self.extension_agent_factory = extension_agent_factory
        self.thread_repository = thread_repository

    def create_agent(self, dto: CreateAgentDto) -> Runnable:
        llm = self.create_llm(dto)
        factory = self.default_agent_factory.create
        assistant_id = self._get_assistant_id(dto)

        if self.extension_agent_factory.is_assistant_an_extension(
            assistant_id=assistant_id
        ):
            factory = self.extension_agent_factory.create

        return factory(
            assistant_id=assistant_id,
            llm=llm,
        )

    def create_llm(self, dto: CreateAgentDto) -> BaseChatModel:
        return self.llm_factory.create_chat_model(
            vendor_model=dto.model,
            max_tokens=dto.max_tokens,
            temperature=dto.temperature,
        )

    def _get_assistant_id(self, dto: CreateAgentDto) -> str:
        thread = self.thread_repository.retreive(thread_id=dto.thread_id)
        thread_assistant_id = None
        if thread is not None:
            thread_assistant_id = thread.metadata.get("assistantId", None)

        return thread_assistant_id or dto.assistant_id
