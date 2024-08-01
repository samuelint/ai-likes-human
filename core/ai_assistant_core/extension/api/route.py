from typing import Optional
from fastapi import APIRouter, HTTPException, UploadFile, File

from fastapi_injector import Injected
from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto
from ai_assistant_core.extension.domain.invalid_file_format_error import (
    InvalidFileFormat,
)
from ai_assistant_core.extension.infrastructure.whl_extension_install_service import (
    WhlExtensionInstallService,
)

extension_router = APIRouter(prefix="/extension")


@extension_router.get("/")
async def list_extensions(
    extension_service: WhlExtensionInstallService = Injected(
        WhlExtensionInstallService
    ),
) -> list[ExtensionInfoDto]:
    return extension_service.list_installed()


@extension_router.post("/upload")
async def upload_extension(
    file: UploadFile = File(...),
    extension_service: WhlExtensionInstallService = Injected(
        WhlExtensionInstallService
    ),
) -> ExtensionInfoDto:

    try:
        return extension_service.upload(filename=file.filename, file=file.file)
    except InvalidFileFormat as e:
        raise HTTPException(status_code=400, detail=str(e))


@extension_router.get("/{name}")
async def find_extension_by_name(
    name: str,
    extension_service: WhlExtensionInstallService = Injected(
        WhlExtensionInstallService
    ),
) -> Optional[ExtensionInfoDto]:
    return extension_service.find_by_name(name=name)


@extension_router.delete("/{name}")
async def delete_extension(
    name: str,
    extension_service: WhlExtensionInstallService = Injected(
        WhlExtensionInstallService
    ),
) -> ExtensionInfoDto:
    try:
        return extension_service.uninstall_by_name(name=name)
    except Exception:
        raise HTTPException(status_code=400, detail="Cannot delete extension")
