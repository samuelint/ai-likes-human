from dataclasses import asdict, dataclass
from typing import Optional


@dataclass
class ExtensionInfoDto:
    name: str
    version: Optional[str]
    author: Optional[str]
    uri: str

    def to_dict(self) -> dict:
        return asdict(self)
