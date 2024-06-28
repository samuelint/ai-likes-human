from typing import List
from fastapi.testclient import TestClient
from openai.types.beta import AssistantStreamEvent
from ai_assistant_core.main import create_app

database_url = "sqlite:///:memory:?check_same_thread=False"


def create_test_client() -> TestClient:
    app = create_app(database_url=database_url)

    return TestClient(app)


def assistant_stream_events_to_str_response(
    stream_response_events: List[AssistantStreamEvent],
) -> str:
    str_response = ""
    for event in stream_response_events:
        if event.event == "thread.message.delta":
            str_response += "".join(event.data.delta.content[0].text.value)

    return str_response
