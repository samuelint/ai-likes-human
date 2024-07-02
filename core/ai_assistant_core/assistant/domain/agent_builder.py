from typing import Optional
from .assistant_prompt_builder import AssistantPromptBuilder
from langchain_core.language_models import BaseChatModel
from langchain_core.tools import BaseTool
from langgraph.prebuilt import create_react_agent


class AgentBuilder:
    def __init__(
        self, llm: BaseChatModel, tools: Optional[list[BaseTool]] = None
    ) -> None:
        self.llm = llm
        self.tools = tools or []

    def build(self):
        prompt_builder = AssistantPromptBuilder(person_name="Samuel Magny")
        system_prompt = prompt_builder.build_system_prompt()

        return create_react_agent(
            self.llm, tools=self.tools, messages_modifier=system_prompt
        )
