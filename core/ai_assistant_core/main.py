from typing import Optional
import uvicorn
import argparse
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from injector import Injector
from fastapi_injector import attach_injector

from ai_assistant_core.app_configuration import (
    AppConfigurationModule,
)
from ai_assistant_core.assistant import AssistantModule, bind_assistant_routes
from ai_assistant_core.configuration.module import ConfigurationModule
from ai_assistant_core.health.route import bind_health_routes
from ai_assistant_core.infrastructure.sqlalchemy_module import SqlAlchemyModule
from ai_assistant_core.configuration import configuration_kv_router
from ai_assistant_core.llm.module import LLMModule


def create_app(database_url: Optional[str] = None) -> FastAPI:
    injector = Injector(
        [
            AppConfigurationModule(database_url=database_url),
            ConfigurationModule(),
            LLMModule(),
            AssistantModule(),
            SqlAlchemyModule(),
        ]
    )

    app = FastAPI(
        title="AI Assistant Core",
        version="1.0",
    )
    app.add_middleware(
        CORSMiddleware,
        allow_origins=["*"],
        allow_methods=["*"],
        allow_headers=["*"],
        expose_headers=["*"],
    )

    attach_injector(app, injector)

    bind_health_routes(app=app)
    bind_assistant_routes(app=app, injector=injector)
    app.include_router(configuration_kv_router)

    return app


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--port", type=int, default=8000, help="Port number")
    args = parser.parse_args()

    app = create_app()
    uvicorn.run(app, host="localhost", port=args.port)
