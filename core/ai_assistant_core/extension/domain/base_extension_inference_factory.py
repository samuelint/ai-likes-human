from abc import ABC, abstractmethod
from ai_assistant_core.extension.domain.inferable_extension import InferableExtension


class BaseExtensionInferenceFactory(ABC):
    @abstractmethod
    def create(
        self,
        extension_name: str,
        extension_llm_model: str,
    ) -> InferableExtension:
        pass
