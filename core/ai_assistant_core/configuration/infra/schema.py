from sqlalchemy import Column, Integer, String

from ai_assistant_core.infrastructure.sqlalchemy import Base
from ai_assistant_core.configuration.domain.dto import ConfigurationItemDto


class ConfigurationItemModel(Base):
    __tablename__ = "configuration_item"

    id = Column(Integer, primary_key=True)
    key = Column(String, unique=True, index=True)
    value = Column(String)

    @staticmethod
    def from_dto(dto: ConfigurationItemDto) -> "ConfigurationItemModel":
        return ConfigurationItemModel(id=dto.id, key=dto.key, value=dto.value)

    def to_dto(self) -> ConfigurationItemDto:
        return ConfigurationItemDto(id=self.id, key=self.key, value=self.value)
