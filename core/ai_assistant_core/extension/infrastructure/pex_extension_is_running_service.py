import time
from typing import Optional
from injector import inject
from ai_assistant_core.extension.infrastructure.pex_extension_api_factory import (
    PexExtensionApiFactory,
)


@inject
class PexExtensionIsRunningService:
    def __init__(
        self,
        extension_api_factory: PexExtensionApiFactory,
    ) -> None:
        self.extension_api_factory = extension_api_factory

    def is_running(
        self,
        extension_name: str,
    ) -> bool:
        try:
            api = self.extension_api_factory.create_from_extension_name(
                extension_name=extension_name
            )

            return api.is_up()

        except Exception:
            return False

    def wait_for_running(
        self,
        extension_name: str,
        timeout_sec: Optional[int] = None,
        delay_sec: Optional[int] = None,
    ) -> bool:
        timeout_sec = timeout_sec or 10
        delay_sec = delay_sec or 0.5

        start_time = time.time()
        while not self.is_running(extension_name):
            if time.time() - start_time > timeout_sec:
                raise TimeoutError(
                    f"Extension {extension_name} was not up after {timeout_sec} seconds"
                )
            time.sleep(delay_sec)

        return True
