from injector import inject
from ai_assistant_core.configuration.domain.repository import ConfigurationRepository


class ApiKeyService:
    @inject
    def __init__(self, configuration_repo: ConfigurationRepository):
        self.configuration_repo = configuration_repo

    def get_openai_api_key(self) -> str:
        return self.configuration_repo.get("OPENAI_API_KEY")

    def get_anthropic_api_key(self) -> str:
        return self.configuration_repo.get("ANTHROPIC_API_KEY")
