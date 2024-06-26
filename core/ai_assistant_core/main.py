import uvicorn
import argparse
from dotenv import load_dotenv, find_dotenv
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

_ = load_dotenv(find_dotenv())


app = FastAPI(
    title="AI Assistant Core",
    version="1.0",
    description="AI Assistant Core API",
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
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

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--port", type=int, default=8000, help="Port number")
    args = parser.parse_args()

    uvicorn.run(app, host="localhost", port=args.port)
