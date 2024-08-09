from langchain_core.runnables import Runnable


class InferableExtension:
    name: str
    description: str
    runnable: Runnable

    def __init__(
        self,
        name: str,
        description: str,
        runnable: Runnable,
    ) -> None:
        self.name = name
        self.description = description
        self.runnable = runnable
