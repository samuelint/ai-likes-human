from injector import inject
from ai_assistant_core.extension.infrastructure.extension_ipc_port_service import (
    ExtensionIpcPortService,
)
from ai_assistant_core.extension.infrastructure.pex_extension_inference_url_factory import (
    PexExtensionInferenceUrlFactory,
)
from ai_assistant_core.extension.infrastructure.pex_installed_extension_repository import (
    PexInstalledExtensionRepository,
)
from ai_assistant_core.extension.infrastructure.pex_process import PexProcess


@inject
class PexProcessFactory:
    def __init__(
        self,
        installed_extensions_repository: PexInstalledExtensionRepository,
        inference_url_factory: PexExtensionInferenceUrlFactory,
    ) -> None:
        self.installed_extensions_repository = installed_extensions_repository
        self.inference_url_factory = inference_url_factory

    def create(
        self,
        extension_name: str,
    ) -> PexProcess:
        extension_info = self.installed_extensions_repository.get_by_name(
            name=extension_name
        )
        ipc_port = ExtensionIpcPortService.find_next_available_port()
        inference_url = self.inference_url_factory.get_self_inference_url()

        return PexProcess(
            pex_path=extension_info.uri,
            ipc_port=ipc_port,
            inference_url=inference_url,
        )
