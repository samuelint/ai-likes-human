from base_assistant_extension import ExtensionMetadata
import requests


class PexExtensionApi:
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def get_metadata(self) -> ExtensionMetadata:
        metadata_url = f"{self.uri}/metadata"
        response = requests.get(metadata_url)
        return response.json()
