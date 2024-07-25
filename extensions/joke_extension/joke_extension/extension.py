from typing import AsyncIterator
from langchain_core.language_models import BaseChatModel
from langchain_core.messages import BaseMessage, BaseMessageChunk, HumanMessage

from base_assistant_extension.base_extension import (
    BaseExtension,
    ExtensionInput,
)


class Extension(BaseExtension):
    def name(self) -> str:
        return "joker"

    def description(self) -> str:
        return "Tell jokes."

    def astream(
        self, llm: BaseChatModel, input: ExtensionInput
    ) -> AsyncIterator[BaseMessageChunk]:
        return llm.astream(self.__generate_messages(input))

    def ainvoke(self, llm: BaseChatModel, input: ExtensionInput) -> BaseMessage:
        return llm.ainvoke(self.__generate_messages(input))

    def __generate_messages(self, input: ExtensionInput) -> list[BaseMessage]:
        messages = input["messages"]
        messages.append(HumanMessage(content="Tell me a joke"))

        return messages
