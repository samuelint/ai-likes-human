from injector import inject
from pydantic import BaseModel
from ai_assistant_core.configuration.domain.repository import ConfigurationRepository


class UserInfo(BaseModel):
    name: str


@inject
class UserInfoService:
    def __init__(self, configuration_repo: ConfigurationRepository) -> None:
        self.configuration_repo = configuration_repo

    def get(self) -> UserInfo:
        kv = self.configuration_repo.get("USERNAME")

        return UserInfo(name=kv.value)
