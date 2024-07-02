from typing import Optional
from pydantic import BaseModel


class ConfigurationItemDto(BaseModel):
    id: Optional[int]
    key: str
    value: str

    class Config:
        orm_mode = True
