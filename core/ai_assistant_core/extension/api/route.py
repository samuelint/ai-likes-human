from typing import Optional
from fastapi import APIRouter, HTTPException, UploadFile, File

from fastapi_injector import Injected
from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto
from ai_assistant_core.extension.domain.invalid_file_format_error import (
    InvalidFileFormat,
)
from ai_assistant_core.extension.infrastructure.whl_extension_service import (
    WhlExtensionService,
)

extension_router = APIRouter(prefix="/extension")

# extensions_dir = os.path.expanduser("~/Desktop/extensions")
# @extension_router.get("/", response_class=PlainTextResponse)
# async def list_extensions(llm_factory: LLMFactory = Injected(LLMFactory)) -> str:
#     wheel_path = os.path.join(extensions_dir, "joke_extension-0.1.0-py3-none-any.whl")
#     joke_extension = WhlExtensionLoader(wheel_path=wheel_path)

#     meta = joke_extension.read_metadata_as_map()
#     print(meta)
#     try:
#         llm = llm_factory.create_chat_model(vendor_model="openai:gpt-4o")
#         extension = joke_extension.load()
#         result = await extension.ainvoke(
#             llm=llm, input={"messages": [HumanMessage(content="About cats")]}
#         )
#         return result.content
#     except Exception as e:
#         return str(e)


@extension_router.get("/")
async def list_extensions(
    extension_service: WhlExtensionService = Injected(WhlExtensionService),
) -> list[ExtensionInfoDto]:
    return extension_service.list_available()


@extension_router.post("/upload")
async def upload_extension(
    file: UploadFile = File(...),
    extension_service: WhlExtensionService = Injected(WhlExtensionService),
) -> ExtensionInfoDto:

    try:
        return extension_service.upload(filename=file.filename, file=file.file)
    except InvalidFileFormat as e:
        raise HTTPException(status_code=400, detail=str(e))


@extension_router.get("/{name}")
async def find_extension_by_name(
    name: str,
    extension_service: WhlExtensionService = Injected(WhlExtensionService),
) -> Optional[ExtensionInfoDto]:
    return extension_service.find_by_name(name=name)


@extension_router.delete("/{name}")
async def delete_extension(
    name: str,
    extension_service: WhlExtensionService = Injected(WhlExtensionService),
) -> ExtensionInfoDto:
    return extension_service.delete(name=name)
