from langchain_core.language_models import BaseChatModel
from langchain_core.runnables import Runnable
from base_assistant_extension.base_extension import (
    BaseExtension,
)
from langchain_core.prompts import PromptTemplate


class Extension(BaseExtension):
    def name(self) -> str:
        return "joker"

    def description(self) -> str:
        return "Tell jokes."

    def create_runnable(self, llm: BaseChatModel) -> Runnable:
        prompt = PromptTemplate.from_template(
            "You tell jokes. No matter the question. You have to tell a joke."
            "{messages}"
        )

        return prompt | llm
