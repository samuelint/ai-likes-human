from injector import inject
from ai_assistant_core.app_configuration import AppConfiguration


@inject
class PexExtensionInferenceUrlFactory:
    def __init__(self, app_configuration: AppConfiguration) -> None:
        self.app_configuration = app_configuration

    def get_self_inference_url(self) -> str:
        return f"{self.app_configuration.self_url}/openai/v1"
