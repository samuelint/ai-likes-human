import pytest
import logging
from openai import OpenAI
from openai.types.beta import Thread
from tests.test_functional.functional_test_utils import create_test_client


test_api = create_test_client()
logging.basicConfig(level=logging.DEBUG)


@pytest.fixture(scope="session")
def openai_client():
    return OpenAI(
        base_url="http://testserver/assistant/openai/v1",
        http_client=test_api,
    )


class TestThreadCRUD:
    def test_create_thread(self, openai_client: OpenAI) -> Thread:
        thread = openai_client.beta.threads.create()

        assert isinstance(thread.id, str)

    def test_create_thread_with_metadata(self, openai_client: OpenAI) -> Thread:
        thread = openai_client.beta.threads.create(metadata={"key": "value"})

        assert thread.metadata["key"] == "value"

    def test_retreive(self, openai_client: OpenAI) -> Thread:
        thread = openai_client.beta.threads.create()

        retreived_thread = openai_client.beta.threads.retrieve(thread_id=thread.id)
        assert retreived_thread == thread

    def test_delete(self, openai_client: OpenAI) -> Thread:
        thread = openai_client.beta.threads.create()

        openai_client.beta.threads.delete(thread_id=thread.id)

        retreived_thread = openai_client.beta.threads.retrieve(thread_id=thread.id)
        assert retreived_thread is None

    def test_update(self, openai_client: OpenAI) -> Thread:
        thread = openai_client.beta.threads.create(metadata={"key": "value"})

        thread = openai_client.beta.threads.update(
            thread_id=thread.id, metadata={"key": "value2"}
        )

        retreived_thread = openai_client.beta.threads.retrieve(thread_id=thread.id)
        assert retreived_thread.metadata == {"key": "value2"}
