from polyfactory.factories import DataclassFactory

from ai_assistant_core.extension.domain.extension_dto import ExtensionInfoDto


class ExtensionInfoDtoPolyfactory(DataclassFactory[ExtensionInfoDto]):
    __model__ = ExtensionInfoDto
