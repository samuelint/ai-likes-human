import pytest
import logging
from openai import OpenAI
from tests.test_functional.functional_test_utils import create_test_client


test_api = create_test_client()
logging.basicConfig(level=logging.DEBUG)


@pytest.fixture(scope="session")
def openai_client():
    return OpenAI(
        base_url="http://testserver/openai/v1",
        http_client=test_api,
    )


class TestInvoke:
    def test_invoke(self, openai_client: OpenAI):
        result = openai_client.chat.completions.create(
            model="openai:gpt-4o-mini",
            messages=[
                {
                    "role": "user",
                    "content": 'Say: "This is a test message."',
                },
            ],
        )

        assert len(result.choices[0].message.content) > 0
