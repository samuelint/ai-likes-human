from typing import Optional
from fastapi import APIRouter, HTTPException, UploadFile, File

from fastapi_injector import Injected
from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto
from ai_assistant_core.extension.domain.extension_state_dto import ExtensionStateDto
from ai_assistant_core.extension.infrastructure.pex_extension_state_service import (
    PexExtensionStateService,
)
from ai_assistant_core.extension.domain.invalid_file_format_error import (
    InvalidFileFormat,
)
from ai_assistant_core.extension.infrastructure.pex_extension_install_service import (
    PexExtensionInstallService,
)
from ai_assistant_core.extension.infrastructure.pex_extension_load_service import (
    PexExtensionLoadService,
)
from ai_assistant_core.extension.infrastructure.pex_extension_uninstall_service import (
    PexExtensionUninstallService,
)

extension_router = APIRouter(prefix="/extension")


@extension_router.get("/")
async def list_extensions(
    extension_state_service: PexExtensionStateService = Injected(
        PexExtensionStateService
    ),
) -> list[ExtensionInfoDto]:
    return extension_state_service.list()


@extension_router.post("/pex/upload")
async def upload_extension(
    file: UploadFile = File(...),
    install_service: PexExtensionInstallService = Injected(PexExtensionInstallService),
    extension_service: PexExtensionStateService = Injected(PexExtensionStateService),
) -> ExtensionStateDto:

    try:
        extension = install_service.install(filename=file.filename, file=file.file)
        return extension_service.find_by_name(name=extension.name)
    except InvalidFileFormat as e:
        raise HTTPException(status_code=400, detail=str(e))


@extension_router.get("/{name}")
async def find_extension_by_name(
    name: str,
    extension_service: PexExtensionStateService = Injected(PexExtensionStateService),
) -> Optional[ExtensionStateDto]:
    return extension_service.find_by_name(name=name)


@extension_router.delete("/{name}")
async def delete_extension(
    name: str,
    extension_service: PexExtensionUninstallService = Injected(
        PexExtensionUninstallService
    ),
) -> ExtensionInfoDto:
    try:
        return extension_service.uninstall_by_name(name=name)
    except Exception:
        raise HTTPException(status_code=400, detail="Cannot delete extension")


@extension_router.post("/{name}/load")
async def load_extension(
    name: str,
    extension_load_service: PexExtensionLoadService = Injected(PexExtensionLoadService),
    extension_state_service: PexExtensionStateService = Injected(
        PexExtensionStateService
    ),
) -> ExtensionInfoDto:
    extension_load_service.load(extension_name=name)

    return extension_state_service.find_by_name(name=name)


@extension_router.post("/{name}/unload")
async def unload_extension(
    name: str,
    extension_load_service: PexExtensionLoadService = Injected(PexExtensionLoadService),
    extension_state_service: PexExtensionStateService = Injected(
        PexExtensionStateService
    ),
) -> ExtensionInfoDto:
    extension_load_service.unload(extension_name=name)

    return extension_state_service.find_by_name(name=name)
