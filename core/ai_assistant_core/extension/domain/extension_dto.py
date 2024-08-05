from dataclasses import asdict, dataclass


@dataclass
class ExtensionInfoDto:
    name: str
    version: str
    author: str
    uri: str

    def to_dict(self) -> dict:
        return asdict(self)
