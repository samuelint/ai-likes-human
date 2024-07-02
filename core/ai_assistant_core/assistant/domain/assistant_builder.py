from .assistant_prompt_builder import AssistantPromptBuilder
from langchain_core.language_models import BaseChatModel


from ai_assistant_core.tools import (
    magic_number_tool,
    web_search_tool,
    dall_e_tool,
    token_size_tool,
    url_content_loader_tool,
)
from langgraph.prebuilt import create_react_agent


agent_tools = [
    magic_number_tool,
    web_search_tool,
    dall_e_tool,
    token_size_tool,
    url_content_loader_tool,
]


class AssistantBuilder:
    def __init__(self, llm: BaseChatModel):
        self.llm = llm

    def build(self):
        prompt_builder = AssistantPromptBuilder(
            person_name="Samuel Magny", confidence_percentage=True
        )
        system_prompt = prompt_builder.build_system_prompt()

        return create_react_agent(
            self.llm, agent_tools, messages_modifier=system_prompt
        )
