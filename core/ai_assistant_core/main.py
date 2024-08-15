from typing import Optional
import uvicorn
import argparse
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from injector import Injector
from fastapi_injector import attach_injector

from ai_assistant_core.assistant import AssistantModule, bind_assistant_routes
from ai_assistant_core.health.route import bind_health_routes
from ai_assistant_core.configuration import routes

from ai_assistant_core.infrastructure import SqlAlchemyModule
from ai_assistant_core.app_configuration import (
    AppConfigurationModule,
)
from ai_assistant_core.configuration import ConfigurationModule
from ai_assistant_core.llm import LLMModule, configuration_local_model_router
from ai_assistant_core.extension import extension_router, ExtensionModule
from ai_assistant_core.tools import ToolsModule


def create_app(
    server_port: int,
    self_host: str,
    database_url: Optional[str] = None,
) -> FastAPI:
    injector = Injector(
        [
            AppConfigurationModule(
                database_url=database_url,
                server_port=server_port,
                self_host=self_host,
            ),
            ConfigurationModule(),
            LLMModule(),
            AssistantModule(),
            ToolsModule(),
            SqlAlchemyModule(),
            ExtensionModule(),
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
    [app.include_router(route) for route in routes]
    app.include_router(configuration_local_model_router)
    app.include_router(extension_router)

    return app


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--port", type=int, default=8000, help="Port number")
    args = parser.parse_args()

    server_port = args.port
    host = "localhost"

    app = create_app(self_host=host, server_port=server_port)
    uvicorn.run(app, host=host, port=server_port)
