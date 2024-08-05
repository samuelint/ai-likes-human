from dataclasses import asdict, dataclass


@dataclass
class ExtensionLoadStateDto:
    pid: int
    name: str

    def to_dict(self) -> dict:
        return asdict(self)
