from fastapi import APIRouter
from fastapi_injector import Injected

from ai_assistant_core.configuration.domain.dto import ConfigurationItemDto
from ai_assistant_core.llm.domain.local_model_service import LocalLLMModelService

configuration_local_model_router = APIRouter(prefix="/configuration/llm/local")


@configuration_local_model_router.get("/")
def list_local_models(
    service: LocalLLMModelService = Injected(LocalLLMModelService),
) -> list[ConfigurationItemDto]:
    items = service.list()

    return items
