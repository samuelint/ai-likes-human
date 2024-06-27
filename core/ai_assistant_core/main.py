import uvicorn
import argparse
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import PlainTextResponse
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
from ai_assistant_core.configuration import Configuration

Configuration.load()

app = FastAPI(
    title="AI Assistant Core",
    version="1.0",
    description="AI Assistant Core API",
)


app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    # allow_credentials=True,
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


@app.get("/health", response_class=PlainTextResponse)
async def health():
    return "healthy"


include_assistant(app=app, assistant_app=assistant_app, prefix="/assistant")


def start_api_server():
    try:
        print("Starting API server...")
        parser = argparse.ArgumentParser()
        parser.add_argument("--port", type=int, default=8000, help="Port number")
        args = parser.parse_args()
        uvicorn.run(app, host="0.0.0.0", port=args.port, log_level="info")
        return True
    except Exception as e:
        print("Failed to start API server")
        print(e)
        return False


if __name__ == "__main__":
    start_api_server()
