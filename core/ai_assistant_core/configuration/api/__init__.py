from .route import configuration_kv_router
from .available_models_route import configuration_available_models_router

routes = [configuration_kv_router, configuration_available_models_router]


__all__ = [
    "configuration_kv_router",
    "configuration_available_models_router",
    "routes",
]
