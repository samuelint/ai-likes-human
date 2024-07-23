from injector import inject
from ai_assistant_core.configuration.domain.repository import ConfigurationRepository


class AvailableModelService:
    @inject
    def __init__(self, repository: ConfigurationRepository):
        self.repository = repository

    def get_available_models(self) -> list[str]:
        items = self.repository.get("AVAILABLE_MODELS").value.split(";")

        return items
