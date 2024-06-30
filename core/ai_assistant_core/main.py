from typing import Optional
import uvicorn
import argparse
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from injector import Injector
from fastapi_injector import attach_injector
from langchain_openai_api_bridge.assistant.assistant_app import AssistantApp
from langchain_openai_api_bridge.assistant.repository import (
    InMemoryMessageRepository,
    InMemoryRunRepository,
    InMemoryThreadRepository,
)
from langchain_openai_api_bridge.fastapi import (
    include_assistant,
)
from ai_assistant_core.assistant_agent_factory import AssistantAgentFactory
from ai_assistant_core.app_configuration import (
    AppConfigurationModule,
)
from ai_assistant_core.configuration.module import LLMConfigurationModule
from ai_assistant_core.infrastructure.sqlalchemy_module import SqlAlchemyModule
from ai_assistant_core.configuration import configuration_kv_router


def create_app(database_url: Optional[str] = None) -> FastAPI:
    injector = Injector(
        [
            AppConfigurationModule(database_url=database_url),
            LLMConfigurationModule(),
            SqlAlchemyModule(),
        ]
    )

    app = FastAPI(
        title="AI Assistant Core",
        version="1.0",
        description="AI Assistant Core API",
    )
    attach_injector(app, injector)

    app.add_middleware(
        CORSMiddleware,
        allow_origins=["*"],
        allow_methods=["*"],
        allow_headers=["*"],
        expose_headers=["*"],
    )

    assistant_app = AssistantApp(
        thread_repository_type=InMemoryThreadRepository,
        message_repository_type=InMemoryMessageRepository,
        run_repository=InMemoryRunRepository,
        agent_factory=AssistantAgentFactory,
    )

    @app.get("/health")
    @app.get("/")
    async def health():
        return {"status": "ok"}

    include_assistant(app=app, assistant_app=assistant_app, prefix="/assistant")
    app.include_router(configuration_kv_router)

    return app


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--port", type=int, default=8000, help="Port number")
    args = parser.parse_args()

    app = create_app()
    uvicorn.run(app, host="localhost", port=args.port)
