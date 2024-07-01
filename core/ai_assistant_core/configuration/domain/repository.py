from abc import ABC, abstractmethod
from .dto import ConfigurationItemDto


class ConfigurationRepository(ABC):
    @abstractmethod
    def get(self, key: str) -> ConfigurationItemDto:
        pass

    @abstractmethod
    def create(self, item: ConfigurationItemDto) -> ConfigurationItemDto:
        pass

    @abstractmethod
    def update(self, item: ConfigurationItemDto) -> ConfigurationItemDto:
        pass

    @abstractmethod
    def list(self, skip: int = 0, limit: int = 100) -> list[ConfigurationItemDto]:
        pass

    @abstractmethod
    def delete(self, key: str) -> ConfigurationItemDto:
        pass
