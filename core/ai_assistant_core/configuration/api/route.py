from fastapi import APIRouter
from fastapi_injector import Injected

from ai_assistant_core.configuration.domain.dto import ConfigurationItemDto
from ai_assistant_core.configuration.domain.repository import (
    ConfigurationRepository,
)

configuration_kv_router = APIRouter(prefix="/configuration/kv")


@configuration_kv_router.get("/")
def list_items(
    config_repository: ConfigurationRepository = Injected(ConfigurationRepository),
) -> list[ConfigurationItemDto]:
    items = config_repository.list()

    return items


@configuration_kv_router.get("/{key}")
def get_item(
    key: str,
    config_repository: ConfigurationRepository = Injected(ConfigurationRepository),
) -> ConfigurationItemDto:
    return config_repository.get(key=key)


@configuration_kv_router.put("/")
def create_item(
    item: ConfigurationItemDto,
    config_repository: ConfigurationRepository = Injected(ConfigurationRepository),
) -> ConfigurationItemDto:
    return config_repository.create(item)


@configuration_kv_router.put("/{key}")
def update_item(
    key: str,
    item: ConfigurationItemDto,
    config_repository: ConfigurationRepository = Injected(ConfigurationRepository),
) -> ConfigurationItemDto:
    item.key = key
    return config_repository.update(item)


@configuration_kv_router.delete("/{key}")
def delete_item(
    key: str,
    config_repository: ConfigurationRepository = Injected(ConfigurationRepository),
) -> ConfigurationItemDto:
    return config_repository.delete(key=key)
