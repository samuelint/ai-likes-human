from injector import Module, multiprovider
from langchain_core.tools import BaseTool

from ai_assistant_core.tools.image_generation.dalle import DallEToolFactory
from ai_assistant_core.tools.magic_number import (
    magic_number_tool,
)
from ai_assistant_core.tools.web_search.web_search import WebSearchToolFactory
from ai_assistant_core.tools.webpage_loader.url_content_loader import (
    url_content_loader_tool,
)


class ToolsModule(Module):
    @multiprovider
    def provide_tools(
        self,
        web_search_tool_factory: WebSearchToolFactory,
        dalle_tool_factory: DallEToolFactory,
    ) -> list[BaseTool]:
        return [
            magic_number_tool,
            url_content_loader_tool,
            web_search_tool_factory.create(),
            dalle_tool_factory.create(),
        ]
