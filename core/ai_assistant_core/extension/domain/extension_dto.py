from pydantic import BaseModel


class ExtensionInfoDto(BaseModel):
    name: str
    version: str
    author: str
    uri: str
