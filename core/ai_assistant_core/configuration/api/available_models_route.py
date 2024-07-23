from fastapi import APIRouter
from fastapi_injector import Injected

from ai_assistant_core.configuration.domain.available_model_service import (
    AvailableModelService,
)

configuration_available_models_router = APIRouter(
    prefix="/configuration/available-models"
)


@configuration_available_models_router.get("/")
def list_available_models(
    service: AvailableModelService = Injected(AvailableModelService),
) -> list[str]:
    return service.get_available_models()
