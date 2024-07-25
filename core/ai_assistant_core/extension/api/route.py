from typing import Optional
from fastapi import APIRouter, HTTPException, UploadFile, File

from fastapi_injector import Injected
from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto
from ai_assistant_core.extension.domain.invalid_file_format_error import (
    InvalidFileFormat,
)
from ai_assistant_core.extension.infrastructure.whl_extension_repository import (
    WhlExtensionRepository,
)

extension_router = APIRouter(prefix="/extension")


@extension_router.get("/")
async def list_extensions(
    extension_service: WhlExtensionRepository = Injected(WhlExtensionRepository),
) -> list[ExtensionInfoDto]:
    return extension_service.list_available()


@extension_router.post("/upload")
async def upload_extension(
    file: UploadFile = File(...),
    extension_service: WhlExtensionRepository = Injected(WhlExtensionRepository),
) -> ExtensionInfoDto:

    try:
        return extension_service.upload(filename=file.filename, file=file.file)
    except InvalidFileFormat as e:
        raise HTTPException(status_code=400, detail=str(e))


@extension_router.get("/{name}")
async def find_extension_by_name(
    name: str,
    extension_service: WhlExtensionRepository = Injected(WhlExtensionRepository),
) -> Optional[ExtensionInfoDto]:
    return extension_service.find_by_name(name=name)


@extension_router.delete("/{name}")
async def delete_extension(
    name: str,
    extension_service: WhlExtensionRepository = Injected(WhlExtensionRepository),
) -> ExtensionInfoDto:
    return extension_service.delete(name=name)
