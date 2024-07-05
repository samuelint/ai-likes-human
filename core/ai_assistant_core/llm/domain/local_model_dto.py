from dataclasses import dataclass
from typing import Optional
from llama_cpp import Literal, Union

LocalModelType = Literal["huggingface", "local"]


@dataclass
class LLMModelFileDto:
    q4_gguf_filepath: Optional[str]
    fp16_gguf_filepath: Optional[str]


@dataclass
class NewFileLLMModel:
    name: str
    type: Literal["huggingface"]

    files: LLMModelFileDto


@dataclass
class NewHuggingFaceLLMModel:
    name: str
    type: Literal["huggingface"]
    repo_id: str

    remote_files: LLMModelFileDto


NewLLMModel = Union[NewFileLLMModel, NewHuggingFaceLLMModel]


@dataclass
class FileLLMModelIndex:
    name: str
    type: Literal["local"]

    local_files: LLMModelFileDto


@dataclass
class HuggingFaceLLMModelIndex:
    name: str
    type: Literal["huggingface"]
    repo_id: str

    remote_files: LLMModelFileDto
    local_files: LLMModelFileDto


LLMModelIndex = Union[FileLLMModelIndex, HuggingFaceLLMModelIndex]
