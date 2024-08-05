from langchain_core.runnables import Runnable


class InferableExtension:
    name: str
    description: str
    runnable: Runnable
