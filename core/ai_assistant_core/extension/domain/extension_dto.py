from dataclasses import dataclass


@dataclass
class ExtensionInfoDto:
    name: str
    version: str
    author: str
    uri: str
