from injector import inject
from langchain_community.utilities.dalle_image_generator import DallEAPIWrapper
from langchain_core.tools import BaseTool, StructuredTool

from ai_assistant_core.llm.domain.api_key_service import ApiKeyService


@inject
class DallEToolFactory:
    def __init__(self, api_key_service: ApiKeyService) -> None:
        api_key = api_key_service.get_openai_api_key()
        self.dalle_client = DallEAPIWrapper(api_key=api_key, model="dall-e-3")

    def generate_image(self, query: str) -> str:
        return self.dalle_client.run(query)

    def create(self) -> BaseTool:
        return StructuredTool.from_function(
            func=self.generate_image,
            name="dall_e",
            description="Generate an image using DALL-E.",
        )
