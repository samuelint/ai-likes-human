from base_assistant_extension import ExtensionMetadata
from langchain_openai import ChatOpenAI
import requests


class PexExtensionApi:
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def get_metadata(self) -> ExtensionMetadata:
        metadata_url = f"{self.uri}/metadata"
        response = requests.get(metadata_url)
        return response.json()

    def is_up(self) -> bool:
        try:
            self.get_metadata()
            return True
        except Exception:
            return False

    def get_proxy_openai_chat_client(self, model: str) -> ChatOpenAI:
        return ChatOpenAI(
            base_url=f"{self.uri}/openai/v1",
            model=model,
        )
