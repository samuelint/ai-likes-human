from typing import Iterable
import httpx

from openai import OpenAI
from openai.types.chat import ChatCompletionMessageParam


class ChatAgentClient:
    def __init__(
        self,
        model: str = "openai:gpt-3.5-turbo",
        base_url: str | httpx.URL | None = None,
        http_client: httpx.Client | None = None,
    ):
        self.http_client = http_client
        self.model = model
        self.openai_client = OpenAI(
            base_url=base_url,
            http_client=http_client,
        )

    def invoke(self, messages: Iterable[ChatCompletionMessageParam]):
        return self.openai_client.chat.completions.create(
            messages=messages,
            model=self.model,
        )

    def stream(self, messages: Iterable[ChatCompletionMessageParam]):
        return self.openai_client.chat.completions.create(
            model=self.model,
            messages=messages,
            stream=True,
        )

    def invoke_as_stream(self, messages: Iterable[ChatCompletionMessageParam]):
        chunks = self.stream(messages)

        every_content = []
        for chunk in chunks:
            if chunk.choices and isinstance(chunk.choices[0].delta.content, str):
                every_content.append(chunk.choices[0].delta.content)

        return "".join(every_content)
