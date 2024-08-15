from fastapi import FastAPI
from injector import Injector
from langchain_openai_api_bridge.core import BaseAgentFactory
from langchain_openai_api_bridge.assistant import (
    ThreadRepository,
    MessageRepository,
    RunRepository,
)
from langchain_openai_api_bridge.fastapi import (
    LangchainOpenaiApiBridgeFastAPI,
)


def bind_assistant_routes(app: FastAPI, injector: Injector):

    bridge = LangchainOpenaiApiBridgeFastAPI(
        app=app, agent_factory_provider=lambda: injector.get(BaseAgentFactory)
    )
    bridge.bind_openai_assistant_api(
        thread_repository_provider=lambda: injector.get(ThreadRepository),
        message_repository_provider=lambda: injector.get(MessageRepository),
        run_repository_provider=lambda: injector.get(RunRepository),
        prefix="/assistant",
    )
    bridge.bind_openai_chat_completion(
        prefix="",
    )
