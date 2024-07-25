import pytest
from joke_extension.extension import Extension
from langchain_openai import ChatOpenAI


@pytest.fixture
def llm() -> ChatOpenAI:
    return ChatOpenAI(
        model="gpt-4o-mini",
    )


def test_extension_tell_jokes(llm: ChatOpenAI):
    extension = Extension()
    runnable = extension.create_runnable(llm=llm)

    result = runnable.invoke("About cats")

    assert "cat" in result.content
