from injector import inject

from langchain.pydantic_v1 import BaseModel, Field
from langchain_core.tools import BaseTool, StructuredTool
from langchain_community.utilities import GoogleSerperAPIWrapper

from ai_assistant_core.llm.domain.api_key_service import ApiKeyService


class WebSearchInput(BaseModel):
    query: str = Field(description="query to search")


@inject
class WebSearchToolFactory:
    def __init__(self, api_key_service: ApiKeyService) -> None:
        api_key = api_key_service.get("SERPER_API_KEY")
        self.serper_client = GoogleSerperAPIWrapper(serper_api_key=api_key)

    def search_web(self, query: str) -> str:
        return self.serper_client.run(query)

    def asearch_web(self, query: str) -> str:
        return self.serper_client.arun(query)

    def create(self) -> BaseTool:
        return StructuredTool.from_function(
            func=self.search_web,
            coroutine=self.asearch_web,
            name="search_web",
            description="useful for when you need to answer questions about current events",
        )
