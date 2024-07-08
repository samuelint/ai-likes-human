from typing import Optional
from pydantic import BaseModel


class ConfigurationItemDto(BaseModel):
    id: Optional[int] = None
    key: str
    value: str

    class Config:
        from_attributes = True
