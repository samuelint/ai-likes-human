from typing import List
import pytest
from tests.test_functional.assets.assets import joke_extension_whl_file_path
from tests.test_functional.functional_test_utils import (
    assistant_stream_events_to_str_response,
    create_test_client,
)
from openai import OpenAI
from openai.types.beta import AssistantStreamEvent, Thread

test_api = create_test_client()


class TestExtensionsInference:

    @pytest.fixture(scope="session")
    def openai_client(self):
        return OpenAI(
            base_url="http://testserver/assistant/openai/v1",
            http_client=test_api,
        )

    @pytest.fixture(scope="session")
    def thread(self, openai_client: OpenAI) -> Thread:
        thread = openai_client.beta.threads.create(
            messages=[
                {
                    "role": "user",
                    "content": "About turtle",
                },
            ]
        )

        return thread

    def upload_joke_extension(self):
        extension_name = ""
        with open(joke_extension_whl_file_path, "rb") as file:
            response = test_api.post(
                "/extension/upload",
                files={
                    "file": (
                        "joke_extension-0.1.0-py3-none-any.whl",
                        file,
                        "application/zip",
                    )
                },
            )
            upload_body = response.json()
            extension_name = upload_body["name"]

        return extension_name

    def test_extension_are_inferable_through_assistant_api(
        self,
        openai_client: OpenAI,
        thread: Thread,
    ):
        extension_name = self.upload_joke_extension()

        stream = openai_client.beta.threads.runs.create(
            thread_id=thread.id,
            model="openai:gpt-3.5-turbo",
            assistant_id=extension_name,
            stream=True,
            temperature=0,
        )

        events: List[AssistantStreamEvent] = []
        for event in stream:
            events.append(event)
        str_response = assistant_stream_events_to_str_response(events)

        assert "turtle" in str_response
