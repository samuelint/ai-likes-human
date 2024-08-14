import pytest
from tests.test_functional.functional_test_utils import (
    create_test_client,
)

from openai import OpenAI

test_api = create_test_client()


class TestChatCompletionApi:

    @pytest.fixture(scope="session")
    def openai_client(self):
        return OpenAI(
            base_url="http://testserver/openai/v1",
            http_client=test_api,
            # base_url="http://localhost:8000/openai/v1",
        )

    def test_inference_through_chat_completion_api(
        self,
        openai_client: OpenAI,
    ):
        result = openai_client.chat.completions.create(
            model="openai:gpt-4o-mini",
            temperature=0,
            messages=[
                {
                    "role": "user",
                    "content": "Say Hi!",
                },
            ],
            stream=False,
        )

        str_response = result.choices[0].message.content

        assert len(str_response) > 0
