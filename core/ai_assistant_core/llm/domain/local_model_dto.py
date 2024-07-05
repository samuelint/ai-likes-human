from dataclasses import dataclass
from llama_cpp import Literal, Union

LocalModelType = Literal["huggingface", "local"]


@dataclass
class NewFileLLMModel:
    name: str
    type: Literal["local"]
    local_path: str


@dataclass
class NewHuggingFaceLLMModel:
    name: str
    type: Literal["huggingface"]
    repo_id: str

    remote_path: str


NewLLMModel = Union[NewFileLLMModel, NewHuggingFaceLLMModel]


@dataclass
class FileLLMModelIndex:
    name: str
    type: Literal["local"]
    local_path: str


@dataclass
class HuggingFaceLLMModelIndex:
    name: str
    type: Literal["huggingface"]
    local_path: str

    repo_id: str
    remote_path: str


LLMModelIndex = Union[FileLLMModelIndex, HuggingFaceLLMModelIndex]
