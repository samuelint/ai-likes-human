from dataclasses import dataclass
from typing import Optional

from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto


@dataclass
class ExtensionStateDto(ExtensionInfoDto):
    is_loaded: bool
    ipc_port: Optional[int]
    pid: Optional[int]
