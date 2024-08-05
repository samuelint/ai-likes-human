from langchain_core.runnables import Runnable

from ai_assistant_core.extension.domain.inferable_extension import InferableExtension


class PexExtensionInferenceFactory:
    def __init__(self) -> None:
        pass

    def create(self, name: str) -> InferableExtension:
        runnable = self.create_runnable(name=name)

        return InferableExtension(
            name="Joke",
            description="Joker",
            runnable=runnable,
        )

    def create_runnable(self, name: str) -> Runnable:
        pass
