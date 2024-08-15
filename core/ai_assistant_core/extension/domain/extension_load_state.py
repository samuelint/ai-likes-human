from dataclasses import asdict, dataclass


@dataclass
class ExtensionLoadStateDto:
    pid: int
    name: str

    ipc_port: int

    def to_dict(self) -> dict:
        return asdict(self)
