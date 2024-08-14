import threading
import time
from typing import List
from fastapi.testclient import TestClient
from openai.types.beta import AssistantStreamEvent
import requests
import uvicorn
from ai_assistant_core.main import create_app

database_url = "sqlite:///:memory:?check_same_thread=False"


def create_test_client() -> TestClient:
    app = create_app(
        server_port=8000,
        database_url=database_url,
        self_host="testserver",
    )

    return TestClient(app)


def assert_http_200_within(url: str, timeout_seconds=10) -> None:
    start_time = time.time()
    while time.time() - start_time < timeout_seconds:
        try:
            response = requests.get(url)
            response.raise_for_status()
            return
        except Exception:
            time.sleep(0.5)
    raise Exception(f"Failed to get {url} within {timeout_seconds} seconds")


def _run_server(server_port: int):
    host = "localhost"
    app = create_app(
        server_port=server_port,
        database_url=database_url,
        self_host=host,
    )

    uvicorn.run(app, host=host, port=server_port)


def start_server(server_port: int = 8000) -> threading.Thread:
    server_thread = threading.Thread(target=_run_server, args=(server_port,))
    server_thread.start()

    assert_http_200_within(f"http://localhost:{server_port}")

    return server_thread


def stop_server(server_thread: threading.Thread):
    server_thread.join(timeout=1)


def assistant_stream_events_to_str_response(
    stream_response_events: List[AssistantStreamEvent],
) -> str:
    str_response = ""
    for event in stream_response_events:
        if event.event == "thread.message.delta":
            str_response += "".join(event.data.delta.content[0].text.value)

    return str_response
