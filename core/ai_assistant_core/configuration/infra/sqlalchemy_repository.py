from injector import inject
from sqlalchemy.orm import Session

from ai_assistant_core.configuration.infra.schema import ConfigurationItemModel

from ..domain.repository import ConfigurationRepository
from ..domain.dto import (
    ConfigurationItemDto,
)


class SqlalchemyConfigurationRepository(ConfigurationRepository):
    @inject
    def __init__(self, db: Session):
        self.db = db

    def get(self, key: str) -> ConfigurationItemDto:
        model = (
            self.db.query(ConfigurationItemModel)
            .filter(ConfigurationItemModel.key == key)
            .first()
        )
        if model is None:
            return None
        return model.to_dto()

    def create(self, item: ConfigurationItemDto) -> ConfigurationItemDto:
        model = ConfigurationItemModel.from_dto(item)
        self.db.add(model)
        self.db.commit()
        self.db.refresh(model)

        return model.to_dto()

    def list(self, skip: int = 0, limit: int = 100) -> list[ConfigurationItemDto]:
        items = self.db.query(ConfigurationItemModel).offset(skip).limit(limit).all()

        return [item.to_dto() for item in items]

    def update(self, item: ConfigurationItemDto) -> ConfigurationItemDto:
        model = (
            self.db.query(ConfigurationItemModel)
            .filter(ConfigurationItemModel.key == item.key)
            .first()
        )
        if model:
            model.value = item.value
            self.db.commit()
            self.db.refresh(model)

        return model.to_dto()

    def delete(self, key: str) -> ConfigurationItemDto:
        model = (
            self.db.query(ConfigurationItemModel)
            .filter(ConfigurationItemModel.key == key)
            .first()
        )
        if model:
            self.db.delete(model)
            self.db.commit()

        return model
