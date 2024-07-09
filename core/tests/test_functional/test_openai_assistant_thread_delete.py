import pytest
from openai import OpenAI

from tests.test_functional.functional_test_utils import create_test_client


test_api = create_test_client()


@pytest.fixture(scope="session")
def openai_client():
    return OpenAI(
        base_url="http://testserver/assistant/openai/v1",
        http_client=test_api,
    )


class TestDelete:

    def test_delete_thread(self, openai_client: OpenAI):
        thread = openai_client.beta.threads.create()

        openai_client.beta.threads.delete(thread_id=thread.id)

        retreived_thread = openai_client.beta.threads.retrieve(thread_id=thread.id)
        assert retreived_thread is None

    def test_runs_associated_with_thread_are_deleted(self, openai_client: OpenAI):
        thread = openai_client.beta.threads.create()

        run = openai_client.beta.threads.runs.create(
            assistant_id="assistant1",
            thread_id=thread.id,
            model="openai:any",
            stream=False,
        )

        openai_client.beta.threads.delete(thread_id=thread.id)

        retreive_run = openai_client.beta.threads.runs.retrieve(
            run_id=run.id, thread_id=thread.id
        )

        assert retreive_run is None

    def test_deleting_a_thread_does_not_affect_other_threads_runs(
        self, openai_client: OpenAI
    ):
        thread1 = openai_client.beta.threads.create()
        thread2 = openai_client.beta.threads.create()
        openai_client.beta.threads.runs.create(
            assistant_id="assistant1",
            thread_id=thread1.id,
            model="openai:any",
            stream=False,
        )
        run2 = openai_client.beta.threads.runs.create(
            assistant_id="assistant1",
            thread_id=thread2.id,
            model="openai:any",
            stream=False,
        )

        openai_client.beta.threads.delete(thread_id=thread1.id)

        retreive_run = openai_client.beta.threads.runs.list(thread_id=thread2.id).data

        assert len(retreive_run) == 1
        assert retreive_run[0].id == run2.id

    def test_deleting_a_thread_does_not_affect_other_threads_messages(
        self, openai_client: OpenAI
    ):
        thread1 = openai_client.beta.threads.create()
        thread2 = openai_client.beta.threads.create()
        openai_client.beta.threads.messages.create(
            thread_id=thread1.id, content="hello", role="user"
        )
        message_thread2 = openai_client.beta.threads.messages.create(
            thread_id=thread2.id, content="salut", role="user"
        )

        openai_client.beta.threads.delete(thread_id=thread1.id)

        retreive_messages = openai_client.beta.threads.messages.list(
            thread_id=thread2.id
        ).data

        assert len(retreive_messages) == 1
        assert retreive_messages[-1].id == message_thread2.id

    def test_messages_associated_with_thread_are_deleted(self, openai_client: OpenAI):

        thread = openai_client.beta.threads.create()
        message = openai_client.beta.threads.messages.create(
            thread_id=thread.id, content="hello", role="user"
        )

        openai_client.beta.threads.delete(thread_id=thread.id)

        retreive_message = openai_client.beta.threads.messages.retrieve(
            message_id=message.id, thread_id=thread.id
        )

        assert retreive_message is None
