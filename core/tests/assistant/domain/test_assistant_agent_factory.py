from decoy import Decoy
import pytest
from ai_assistant_core.assistant.domain.agents.default_agent_factory import (
    DefaultAgentFactory,
)
from ai_assistant_core.assistant.domain.agents.extension_agent_factory import (
    ExtensionAgentFactory,
)
from ai_assistant_core.assistant.domain.assistant_agent_factory import (
    AssistantAgentFactory,
)
from ai_assistant_core.llm.domain.llm_factory import LLMFactory
from langchain_openai_api_bridge.assistant import (
    ThreadRepository,
)
from langchain_openai_api_bridge.core.create_agent_dto import CreateAgentDto
from langchain_core.language_models import BaseChatModel
from langchain_core.runnables import Runnable
from openai.types.beta import Thread


class TestAssistantAgentFactory:

    @pytest.fixture
    def llm_factory(self, decoy: Decoy, llm: BaseChatModel) -> LLMFactory:
        llm_factory = decoy.mock(cls=LLMFactory)

        decoy.when(
            llm_factory.create_chat_model(
                vendor_model="some:model",
                max_tokens=100,
                temperature=0.5,
            )
        ).then_return(llm)

        return llm_factory

    @pytest.fixture
    def llm(self, decoy: Decoy) -> BaseChatModel:
        return decoy.mock(cls=BaseChatModel)

    @pytest.fixture
    def some_agent(self, decoy: Decoy) -> Runnable:
        return decoy.mock(cls=Runnable)

    @pytest.fixture
    def default_agent_factory(self, decoy: Decoy) -> DefaultAgentFactory:
        return decoy.mock(cls=DefaultAgentFactory)

    @pytest.fixture
    def extension_agent_factory(self, decoy: Decoy) -> ExtensionAgentFactory:
        return decoy.mock(cls=ExtensionAgentFactory)

    @pytest.fixture
    def thread_repository(self, decoy: Decoy) -> ThreadRepository:
        return decoy.mock(cls=ThreadRepository)

    @pytest.fixture
    def instance(
        self,
        llm_factory,
        default_agent_factory,
        extension_agent_factory,
        thread_repository,
    ) -> AssistantAgentFactory:
        return AssistantAgentFactory(
            llm_factory=llm_factory,
            default_agent_factory=default_agent_factory,
            extension_agent_factory=extension_agent_factory,
            thread_repository=thread_repository,
        )

    def test_without_assistant_id_default_agent_is_created(
        self,
        decoy: Decoy,
        instance: AssistantAgentFactory,
        default_agent_factory: DefaultAgentFactory,
        extension_agent_factory: ExtensionAgentFactory,
        llm: BaseChatModel,
        some_agent: Runnable,
    ):
        decoy.when(
            extension_agent_factory.is_assistant_an_extension(
                assistant_id=None,
            )
        ).then_return(False)
        decoy.when(
            default_agent_factory.create(
                assistant_id=None,
                llm=llm,
            )
        ).then_return(some_agent)

        result = instance.create_agent(
            CreateAgentDto(
                assistant_id=None,
                model="some:model",
                max_tokens=100,
                temperature=0.5,
            )
        )

        assert result is some_agent

    def test_with_empty_string_assistant_id_default_agent_is_created(
        self,
        decoy: Decoy,
        instance: AssistantAgentFactory,
        default_agent_factory: DefaultAgentFactory,
        extension_agent_factory: ExtensionAgentFactory,
        llm: BaseChatModel,
        some_agent: Runnable,
    ):
        decoy.when(
            extension_agent_factory.is_assistant_an_extension(
                assistant_id="",
            )
        ).then_return(False)
        decoy.when(
            default_agent_factory.create(
                assistant_id="",
                llm=llm,
            )
        ).then_return(some_agent)

        result = instance.create_agent(
            CreateAgentDto(
                assistant_id="",
                model="some:model",
                max_tokens=100,
                temperature=0.5,
            )
        )

        assert result is some_agent

    def test_assistant_id_an_existing_extension_then_extension_agent_is_created(
        self,
        decoy: Decoy,
        instance: AssistantAgentFactory,
        extension_agent_factory: ExtensionAgentFactory,
        llm: BaseChatModel,
        some_agent: Runnable,
    ):
        decoy.when(
            extension_agent_factory.is_assistant_an_extension(
                assistant_id="some-extension",
            )
        ).then_return(True)
        decoy.when(
            extension_agent_factory.create(
                assistant_id="some-extension",
                llm=llm,
            )
        ).then_return(some_agent)

        result = instance.create_agent(
            CreateAgentDto(
                assistant_id="some-extension",
                model="some:model",
                max_tokens=100,
                temperature=0.5,
            )
        )

        assert result is some_agent

    def test_thread_id_associated_assistant_id_an_existing_extension_then_extension_agent_is_created(
        self,
        decoy: Decoy,
        instance: AssistantAgentFactory,
        extension_agent_factory: ExtensionAgentFactory,
        thread_repository: ThreadRepository,
        llm: BaseChatModel,
        some_agent: Runnable,
    ):
        some_thread = decoy.mock(cls=Thread)
        some_thread.metadata = {"assistantId": "some-extension"}
        decoy.when(thread_repository.retreive(thread_id="some-thread")).then_return(
            some_thread
        )

        decoy.when(
            extension_agent_factory.is_assistant_an_extension(
                assistant_id="some-extension",
            )
        ).then_return(True)
        decoy.when(
            extension_agent_factory.create(
                assistant_id="some-extension",
                llm=llm,
            )
        ).then_return(some_agent)

        result = instance.create_agent(
            CreateAgentDto(
                assistant_id="some-extension",
                model="some:model",
                max_tokens=100,
                temperature=0.5,
            )
        )

        assert result is some_agent
