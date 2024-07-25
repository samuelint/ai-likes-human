from polyfactory.factories import DataclassFactory
from ai_assistant_core.llm.domain.local_model_dto import (
    FileLLMModelIndex,
)


class LLMModelIndexPolyfactory(DataclassFactory[FileLLMModelIndex]):
    __model__ = FileLLMModelIndex
