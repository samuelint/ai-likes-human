from typing import List
import pytest
import logging
from openai import OpenAI
from openai.types.beta import AssistantStreamEvent, Thread
from tests.test_functional.functional_test_utils import create_test_client

from tests.test_functional.functional_test_utils import (
    assistant_stream_events_to_str_response,
)


test_api = create_test_client()
logging.basicConfig(level=logging.DEBUG)


@pytest.fixture(scope="session")
def openai_client():
    return OpenAI(
        base_url="http://testserver/assistant/openai/v1",
        http_client=test_api,
    )


class TestRunStream:

    @pytest.fixture(scope="session")
    def thread(self, openai_client: OpenAI) -> Thread:
        thread = openai_client.beta.threads.create(
            messages=[
                {
                    "role": "user",
                    "content": 'Say: "This is a test message."',
                },
            ]
        )

        return thread

    @pytest.fixture(scope="session")
    def stream_response_events(
        self, openai_client: OpenAI, thread: Thread
    ) -> List[AssistantStreamEvent]:

        stream = openai_client.beta.threads.runs.create(
            thread_id=thread.id,
            model="openai:gpt-3.5-turbo",
            assistant_id="any",
            stream=True,
            temperature=0,
        )

        events: List[AssistantStreamEvent] = []
        for event in stream:
            events.append(event)

        return events

    def test_run_stream_message_deltas(
        self, stream_response_events: List[AssistantStreamEvent]
    ):
        str_response = assistant_stream_events_to_str_response(stream_response_events)

        assert "This is a test message." in str_response
