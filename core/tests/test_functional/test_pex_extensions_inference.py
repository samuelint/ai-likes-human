import time
import requests
from typing import List
import pytest
from tests.test_functional.assets.assets import (
    joke_extension_pex_file_path,
    joke_extension_pex_file_name,
)
from tests.test_functional.functional_test_utils import (
    assistant_stream_events_to_str_response,
    start_server,
    stop_server,
)

from openai import OpenAI
from openai.types.beta import AssistantStreamEvent, Thread


class TestPexExtensionsInference:

    base_url = "http://localhost:9001"

    @pytest.fixture(scope="session")
    def start_server(self):
        handle = start_server(9001)

        yield

        stop_server(handle)

    @pytest.fixture(scope="session")
    def openai_client(self, start_server):
        return OpenAI(
            base_url=f"{self.base_url}/assistant/openai/v1",
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

    def install_joke_extension(self):
        extension_name = ""
        with open(joke_extension_pex_file_path, "rb") as file:
            response = requests.post(
                f"{self.base_url}/extension/upload",
                files={
                    "file": (
                        joke_extension_pex_file_name,
                        file,
                        "application/zip",
                    )
                },
            )
            upload_body = response.json()
            extension_name = upload_body["name"]

        return extension_name

    def load_extension(self, extension_name: str) -> None:
        requests.post(f"{self.base_url}/extension/{extension_name}/load")

    def test_extension_are_inferable_through_assistant_api(
        self,
        openai_client: OpenAI,
        thread: Thread,
    ):
        extension_name = self.install_joke_extension()
        self.load_extension(extension_name)

        time.sleep(15)

        stream = openai_client.beta.threads.runs.create(
            thread_id=thread.id,
            model="openai:gpt-4o-mini",
            assistant_id=extension_name,
            stream=True,
            temperature=0,
        )

        events: List[AssistantStreamEvent] = []
        for event in stream:
            events.append(event)
        str_response = assistant_stream_events_to_str_response(events)

        grader_response = OpenAI().chat.completions.create(
            model="gpt-4o-mini",
            messages=[
                {
                    "role": "user",
                    "content": f"Is this a joke about turtle? Answer by yes or no. \nMesssage: {str_response}",
                }
            ],
            stream=False,
        )

        assert "yes" in grader_response.choices[0].message.content.lower()
